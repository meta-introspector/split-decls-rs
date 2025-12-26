use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// What type of change is described by a `DiffDelta`?
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Delta {
    /// No changes
    Unmodified,
    /// Entry does not exist in old version
    Added,
    /// Entry does not exist in new version
    Deleted,
    /// Entry content changed between old and new
    Modified,
    /// Entry was renamed between old and new
    Renamed,
    /// Entry was copied from another old entry
    Copied,
    /// Entry is ignored item in workdir
    Ignored,
    /// Entry is untracked item in workdir
    Untracked,
    /// Type of entry changed between old and new
    Typechange,
    /// Entry is unreadable
    Unreadable,
    /// Entry in the index is conflicted
    Conflicted,
}
