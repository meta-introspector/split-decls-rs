// Generated macro for GitRepo (struct)
macro_rules! Depcrate_fs_feature_gitGitRepo {
() => {
// Module: crate::fs::feature::git
// Provides: {"GitRepo"}
// Dependencies: {}
# [doc = " A **Git repository** is one we’ve discovered somewhere on the filesystem."] pub struct GitRepo { # [doc = " The queryable contents of the repository: either a `git2` repo, or the"] # [doc = " cached results from when we queried it last time."] contents : Mutex < GitContents > , # [doc = " The working directory of this repository."] # [doc = " This is used to check whether two repositories are the same."] workdir : PathBuf , # [doc = " The path that was originally checked to discover this repository."] # [doc = " This is as important as the `extra_paths` (it gets checked first), but"] # [doc = " is separate to avoid having to deal with a non-empty Vec."] original_path : PathBuf , # [doc = " Any other paths that were checked only to result in this same"] # [doc = " repository."] extra_paths : Vec < PathBuf > , }
};
}
