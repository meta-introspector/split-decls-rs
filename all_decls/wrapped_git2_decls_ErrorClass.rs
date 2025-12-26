use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration of possible categories of things that can have
/// errors when working with a git repository.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorClass {
    /// Uncategorized
    None,
    /// Out of memory or insufficient allocated space
    NoMemory,
    /// Syscall or standard system library error
    Os,
    /// Invalid input
    Invalid,
    /// Error resolving or manipulating a reference
    Reference,
    /// ZLib failure
    Zlib,
    /// Bad repository state
    Repository,
    /// Bad configuration
    Config,
    /// Regex failure
    Regex,
    /// Bad object
    Odb,
    /// Invalid index data
    Index,
    /// Error creating or obtaining an object
    Object,
    /// Network error
    Net,
    /// Error manipulating a tag
    Tag,
    /// Invalid value in tree
    Tree,
    /// Hashing or packing error
    Indexer,
    /// Error from SSL
    Ssl,
    /// Error involving submodules
    Submodule,
    /// Threading error
    Thread,
    /// Error manipulating a stash
    Stash,
    /// Checkout failure
    Checkout,
    /// Invalid FETCH_HEAD
    FetchHead,
    /// Merge failure
    Merge,
    /// SSH failure
    Ssh,
    /// Error manipulating filters
    Filter,
    /// Error reverting commit
    Revert,
    /// Error from a user callback
    Callback,
    /// Error cherry-picking commit
    CherryPick,
    /// Can't describe object
    Describe,
    /// Error during rebase
    Rebase,
    /// Filesystem-related error
    Filesystem,
    /// Invalid patch data
    Patch,
    /// Error involving worktrees
    Worktree,
    /// Hash library error or SHA-1 collision
    Sha1,
    /// HTTP error
    Http,
}
