use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A single change in a `Diff` that represents a range of tokens (`before`)
/// in the first sequence that were replaced by a different range of tokens
/// in the second sequence (`after`).
///
/// Tokens that are a
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Hunk {
    pub before: Range<u32>,
    pub after: Range<u32>,
}
