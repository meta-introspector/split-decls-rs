// Generated macro for working_tree_status (function)
macro_rules! Depcrate_fs_feature_gitworking_tree_status {
() => {
// Module: crate::fs::feature::git
// Provides: {"working_tree_status"}
// Dependencies: {}
# [doc = " The character to display if the file has been modified, but not staged."] fn working_tree_status (status : git2 :: Status) -> f :: GitStatus { # [rustfmt :: skip] return match status { s if s . contains (git2 :: Status :: WT_NEW) => f :: GitStatus :: New , s if s . contains (git2 :: Status :: WT_MODIFIED) => f :: GitStatus :: Modified , s if s . contains (git2 :: Status :: WT_DELETED) => f :: GitStatus :: Deleted , s if s . contains (git2 :: Status :: WT_RENAMED) => f :: GitStatus :: Renamed , s if s . contains (git2 :: Status :: WT_TYPECHANGE) => f :: GitStatus :: TypeChange , s if s . contains (git2 :: Status :: IGNORED) => f :: GitStatus :: Ignored , s if s . contains (git2 :: Status :: CONFLICTED) => f :: GitStatus :: Conflicted , _ => f :: GitStatus :: NotModified , } ; }
};
}
