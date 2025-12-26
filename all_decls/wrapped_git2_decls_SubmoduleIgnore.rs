use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Submodule ignore values
///
/// These values represent settings for the `submodule.$name.ignore`
/// configuration value which says how deeply to look at the working
/// directory when getting the submodule status.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SubmoduleIgnore {
    /// Use the submodule's configuration
    Unspecified,
    /// Any change or untracked file is considered dirty
    None,
    /// Only dirty if tracked files have changed
    Untracked,
    /// Only dirty if HEAD has moved
    Dirty,
    /// Never dirty
    All,
}
