use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A listing of the possible states that a repository can be in.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
#[allow(missing_docs)]
pub enum RepositoryState {
    Clean,
    Merge,
    Revert,
    RevertSequence,
    CherryPick,
    CherryPickSequence,
    Bisect,
    Rebase,
    RebaseInteractive,
    RebaseMerge,
    ApplyMailbox,
    ApplyMailboxOrRebase,
}
