// Generated macro for new_repo (function)
macro_rules! Depcrate_gitnew_repo {
() => {
// Module: crate::git
// Provides: {"new_repo"}
// Dependencies: {}
# [doc = " Create a new [`Project`] with access to the [`Repository`]"] pub fn new_repo < F > (name : & str , callback : F) -> (Project , git2 :: Repository) where F : FnOnce (ProjectBuilder) -> ProjectBuilder , { let mut git_project = project () . at (name) ; git_project = callback (git_project) ; let git_project = git_project . build () ; let repo = init (& git_project . root ()) ; add (& repo) ; commit (& repo) ; (git_project , repo) }
};
}
