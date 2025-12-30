// Generated macro for get_git_commit (function)
macro_rules! Depcrateget_git_commit {
() => {
// Module: crate
// Provides: {"get_git_commit"}
// Dependencies: {}
# [doc = " Gets the latest git commit of the source code directory."] # [doc = ""] # [doc = " Example: `\"a5547bfb1edb9712588f0f85d3e2c8ba618ac51f\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn get_git_commit () -> Result < String , String > { exec ("git" , & ["rev-parse" , "HEAD"]) }
};
}
