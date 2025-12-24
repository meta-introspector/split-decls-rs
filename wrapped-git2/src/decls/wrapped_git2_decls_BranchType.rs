use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration for the possible types of branches
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BranchType {
    /// A local branch not on a remote.
    Local,
    /// A branch for a remote.
    Remote,
}
