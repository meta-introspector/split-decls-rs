// Generated macro for GitContents (enum)
macro_rules! Depcrate_fs_feature_gitGitContents {
() => {
// Module: crate::fs::feature::git
// Provides: {"GitContents"}
// Dependencies: {}
# [doc = " A repository’s queried state."] enum GitContents { # [doc = " All the interesting Git stuff goes through this."] Before { repo : git2 :: Repository } , # [doc = " Temporary value used in `repo_to_statuses` so we can move the"] # [doc = " repository out of the `Before` variant."] Processing , # [doc = " The data we’ve extracted from the repository, but only after we’ve"] # [doc = " actually done so."] After { statuses : Git } , }
};
}
