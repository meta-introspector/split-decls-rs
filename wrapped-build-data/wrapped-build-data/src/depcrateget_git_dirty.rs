// Generated macro for get_git_dirty (function)
macro_rules! Depcrateget_git_dirty {
() => {
// Module: crate
// Provides: {"get_git_dirty"}
// Dependencies: {}
# [doc = " Returns `true` if the source directory contains uncommitted changes."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn get_git_dirty () -> Result < bool , String > { Ok (! exec ("git" , & ["status" , "-s"]) ? . is_empty ()) }
};
}
