// Generated macro for REPO_GIT_DIR (const)
macro_rules! Depcrate_setup_git_hookREPO_GIT_DIR {
() => {
// Module: crate::setup::git_hook
// Provides: {"REPO_GIT_DIR"}
// Dependencies: {}
# [doc = " Rusts setup uses `git rev-parse --git-common-dir` to get the root directory of the repo."] # [doc = " I've decided against this for the sake of simplicity and to make sure that it doesn't install"] # [doc = " the hook if `clippy_dev` would be used in the rust tree. The hook also references this tool"] # [doc = " for formatting and should therefore only be used in a normal clone of clippy"] const REPO_GIT_DIR : & str = ".git" ;
};
}
