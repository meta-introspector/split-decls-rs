use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration all possible kinds objects may have.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ObjectType {
    /// Any kind of git object
    Any,
    /// An object which corresponds to a git commit
    Commit,
    /// An object which corresponds to a git tree
    Tree,
    /// An object which corresponds to a git blob
    Blob,
    /// An object which corresponds to a git tag
    Tag,
}
