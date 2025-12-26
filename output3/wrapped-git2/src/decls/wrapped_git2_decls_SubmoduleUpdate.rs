use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Submodule update values
///
/// These values represent settings for the `submodule.$name.update`
/// configuration value which says how to handle `git submodule update`
/// for this submodule. The value is usually set in the ".gitmodules"
/// file and copied to ".git/config" when the submodule is initialized.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SubmoduleUpdate {
    /// The default; when a submodule is updated, checkout the new detached
    /// HEAD to the submodule directory.
    Checkout,
    /// Update by rebasing the current checked out branch onto the commit from
    /// the superproject.
    Rebase,
    /// Update by merging the commit in the superproject into the current
    /// checkout out branch of the submodule.
    Merge,
    /// Do not update this submodule even when the commit in the superproject
    /// is updated.
    None,
    /// Not used except as static initializer when we don't want any particular
    /// update rule to be specified.
    Default,
}
