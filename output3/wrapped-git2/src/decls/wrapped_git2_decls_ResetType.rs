use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration of the operations that can be performed for the `reset`
/// method on a `Repository`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResetType {
    /// Move the head to the given commit.
    Soft,
    /// Soft plus reset the index to the commit.
    Mixed,
    /// Mixed plus changes in the working tree are discarded.
    Hard,
}
