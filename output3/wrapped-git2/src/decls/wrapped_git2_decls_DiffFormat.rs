use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Possible output formats for diff data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DiffFormat {
    /// full git diff
    Patch,
    /// just the headers of the patch
    PatchHeader,
    /// like git diff --raw
    Raw,
    /// like git diff --name-only
    NameOnly,
    /// like git diff --name-status
    NameStatus,
    /// git diff as used by git patch-id
    PatchId,
}
