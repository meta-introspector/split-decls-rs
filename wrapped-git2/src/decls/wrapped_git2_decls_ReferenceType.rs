use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration of all possible kinds of references.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ReferenceType {
    /// A reference which points at an object id.
    Direct,
    /// A reference which points at another reference.
    Symbolic,
}
