use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StashApplyProgress {
    /// None
    None,
    /// Loading the stashed data from the object database
    LoadingStash,
    /// The stored index is being analyzed
    AnalyzeIndex,
    /// The modified files are being analyzed
    AnalyzeModified,
    /// The untracked and ignored files are being analyzed
    AnalyzeUntracked,
    /// The untracked files are being written to disk
    CheckoutUntracked,
    /// The modified files are being written to disk
    CheckoutModified,
    /// The stash was applied successfully
    Done,
}
