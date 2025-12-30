// Generated macro for DiffLineKind (enum)
macro_rules! Depcrate_blob_unified_diffDiffLineKind {
() => {
// Module: crate::blob::unified_diff
// Provides: {"DiffLineKind"}
// Dependencies: {}
# [doc = " Represents the type of a line in a unified diff."] # [doc (alias = "git2")] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum DiffLineKind { # [doc = " A line that exists in both the old and the new version, added based on [`ContextSize`]."] Context , # [doc = " A line that was added in the new version."] Add , # [doc = " A line that was removed from the old version."] Remove , }
};
}
