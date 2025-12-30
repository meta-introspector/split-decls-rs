// Generated macro for has_changed_since (function)
macro_rules! Depcrate_githas_changed_since {
() => {
// Module: crate::git
// Provides: {"has_changed_since"}
// Dependencies: {}
# [doc = " Returns true if any of the passed `paths` have changed since the `base` commit."] pub fn has_changed_since (git_dir : & Path , base : & str , paths : & [& str]) -> bool { let mut git = Command :: new ("git") ; git . current_dir (git_dir) ; git . args (["diff-index" , "--quiet" , base , "--"]) . args (paths) ; ! git . status () . expect ("cannot run git diff-index") . success () }
};
}
