use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct TargetTriple(String);

impl TargetTriple {
    #[cfg(test)]
    pub(crate) fn from_bazel(bazel: String) -> Self {
        Self(bazel)
    }

    pub(crate) fn to_bazel(&self) -> String {
        self.0.clone()
    }

    pub(crate) fn to_cargo(&self) -> String {
        // While Bazel is NixOS and DriveLinux aware (via `@platforms//os:nixos` or `@platforms//os:drive_linux`), `rustc`
        // is not, so any target triples for `nixos` and get `drive_linux` remapped to `linux`
        // for the purposes of determining `cargo metadata`, resolving `cfg`
        // targets, etc.
        let target_mapping = [("nixos", "linux"), ("drive_linux", "linux")];
        for (bazel_target, cargo_target) in &target_mapping {
            if self.0.contains(bazel_target) {
                return self.0.replace(bazel_target, cargo_target);
            }
        }
        self.0.clone()
    }
}

impl Display for TargetTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let bazel = self.to_bazel();
        let cargo = self.to_cargo();
        match bazel == cargo {
            true => write!(f, "{}", bazel),
            false => write!(f, "{} (cargo: {})", bazel, cargo),
        }
    }
}
