// Generated macro for new (function)
macro_rules! Depcrate_gitnew {
() => {
// Module: crate::git
// Provides: {"new"}
// Dependencies: {}
# [doc = " Create a new [`Project`] in a git [`Repository`]"] pub fn new < F > (name : & str , callback : F) -> Project where F : FnOnce (ProjectBuilder) -> ProjectBuilder , { new_repo (name , callback) . 0 }
};
}
