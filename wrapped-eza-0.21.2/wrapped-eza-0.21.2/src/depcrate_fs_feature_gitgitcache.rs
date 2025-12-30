// Generated macro for GitCache (struct)
macro_rules! Depcrate_fs_feature_gitGitCache {
() => {
// Module: crate::fs::feature::git
// Provides: {"GitCache"}
// Dependencies: {}
# [doc = " A **Git cache** is assembled based on the user’s input arguments."] # [doc = ""] # [doc = " This uses vectors to avoid the overhead of hashing: it’s not worth it when the"] # [doc = " expected number of Git repositories per exa invocation is 0 or 1..."] pub struct GitCache { # [doc = " A list of discovered Git repositories and their paths."] repos : Vec < GitRepo > , # [doc = " Paths that we’ve confirmed do not have Git repositories underneath them."] misses : Vec < PathBuf > , }
};
}
