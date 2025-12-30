// Generated macro for index_status (function)
macro_rules! Depcrate_fs_feature_gitindex_status {
() => {
// Module: crate::fs::feature::git
// Provides: {"index_status"}
// Dependencies: {}
# [doc = " The character to display if the file has been modified and the change"] # [doc = " has been staged."] fn index_status (status : git2 :: Status) -> f :: GitStatus { # [rustfmt :: skip] return match status { s if s . contains (git2 :: Status :: INDEX_NEW) => f :: GitStatus :: New , s if s . contains (git2 :: Status :: INDEX_MODIFIED) => f :: GitStatus :: Modified , s if s . contains (git2 :: Status :: INDEX_DELETED) => f :: GitStatus :: Deleted , s if s . contains (git2 :: Status :: INDEX_RENAMED) => f :: GitStatus :: Renamed , s if s . contains (git2 :: Status :: INDEX_TYPECHANGE) => f :: GitStatus :: TypeChange , _ => f :: GitStatus :: NotModified , } ; }
};
}
