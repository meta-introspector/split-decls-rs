// Generated macro for get_git_branch (function)
macro_rules! Depcrateget_git_branch {
() => {
// Module: crate
// Provides: {"get_git_branch"}
// Dependencies: {}
# [doc = " Gets the current branch of the source code directory."] # [doc = ""] # [doc = " Example: `\"release\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn get_git_branch () -> Result < String , String > { exec ("git" , & ["rev-parse" , "--abbrev-ref=loose" , "HEAD"]) }
};
}
