// Generated macro for init (function)
macro_rules! Depcrate_gitinit {
() => {
// Module: crate::git
// Provides: {"init"}
// Dependencies: {}
# [doc = " *(`git2`)* Initialize a new repository at the given path."] pub fn init (path : & Path) -> git2 :: Repository { default_search_path () ; let repo = t ! (git2 :: Repository :: init (path)) ; default_repo_cfg (& repo) ; repo }
};
}
