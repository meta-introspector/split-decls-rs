use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Kind of [file change](ChangedFile).
#[derive(Eq, PartialEq, Debug)]
pub enum Change {
    /// The file was (re-)created
    Create(Vec<u8>, u64),
    /// The file was modified
    Modify(Vec<u8>, u64),
    /// The file was deleted
    Delete,
}
