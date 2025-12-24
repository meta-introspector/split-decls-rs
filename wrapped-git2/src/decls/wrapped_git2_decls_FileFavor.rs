use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Merge file favor options for `MergeOptions` instruct the file-level
/// merging functionality how to deal with conflicting regions of the files.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum FileFavor {
    /// When a region of a file is changed in both branches, a conflict will be
    /// recorded in the index so that git_checkout can produce a merge file with
    /// conflict markers in the working directory. This is the default.
    Normal,
    /// When a region of a file is changed in both branches, the file created
    /// in the index will contain the "ours" side of any conflicting region.
    /// The index will not record a conflict.
    Ours,
    /// When a region of a file is changed in both branches, the file created
    /// in the index will contain the "theirs" side of any conflicting region.
    /// The index will not record a conflict.
    Theirs,
    /// When a region of a file is changed in both branches, the file created
    /// in the index will contain each unique line from each side, which has
    /// the result of combining both files. The index will not record a conflict.
    Union,
}
