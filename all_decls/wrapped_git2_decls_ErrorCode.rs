use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration of possible errors that can happen when working with a git
/// repository.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorCode {
    /// Generic error
    GenericError,
    /// Requested object could not be found
    NotFound,
    /// Object exists preventing operation
    Exists,
    /// More than one object matches
    Ambiguous,
    /// Output buffer too short to hold data
    BufSize,
    /// User-generated error
    User,
    /// Operation not allowed on bare repository
    BareRepo,
    /// HEAD refers to branch with no commits
    UnbornBranch,
    /// Merge in progress prevented operation
    Unmerged,
    /// Reference was not fast-forwardable
    NotFastForward,
    /// Name/ref spec was not in a valid format
    InvalidSpec,
    /// Checkout conflicts prevented operation
    Conflict,
    /// Lock file prevented operation
    Locked,
    /// Reference value does not match expected
    Modified,
    /// Authentication error
    Auth,
    /// Server certificate is invalid
    Certificate,
    /// Patch/merge has already been applied
    Applied,
    /// The requested peel operation is not possible
    Peel,
    /// Unexpected EOF
    Eof,
    /// Invalid operation or input
    Invalid,
    /// Uncommitted changes in index prevented operation
    Uncommitted,
    /// Operation was not valid for a directory
    Directory,
    /// A merge conflict exists and cannot continue
    MergeConflict,
    /// Hashsum mismatch in object
    HashsumMismatch,
    /// Unsaved changes in the index would be overwritten
    IndexDirty,
    /// Patch application failed
    ApplyFail,
    /// The object is not owned by the current user
    Owner,
    /// Timeout
    Timeout,
}
