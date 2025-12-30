// Generated macro for get_git_commit_short (function)
macro_rules! Depcrateget_git_commit_short {
() => {
// Module: crate
// Provides: {"get_git_commit_short"}
// Dependencies: {}
# [doc = " Gets the latest git commit of the source code directory."] # [doc = " Returns the truncated hash."] # [doc = ""] # [doc = " Example: `\"a5547bf\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn get_git_commit_short () -> Result < String , String > { let long = get_git_commit () ? ; if long . len () < 7 { return Err (format ! ("got malformed commit hash from git: '{long}'")) ; } let short = & long [0 .. 7] ; Ok (short . to_string ()) }
};
}
