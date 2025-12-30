// Generated macro for set_GIT_COMMIT (function)
macro_rules! Depcrateset_GIT_COMMIT {
() => {
// Module: crate
// Provides: {"set_GIT_COMMIT"}
// Dependencies: {}
# [doc = " Sets the `GIT_COMMIT` env variable."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Example value: `\"a5547bfb1edb9712588f0f85d3e2c8ba618ac51f\"`"] # [doc = ""] # [doc = " Related: [`rerun_if_git_commit_or_branch_changed`]"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn set_GIT_COMMIT () -> Result < () , String > { let value = get_git_commit () ? ; println ! ("cargo:rustc-env=GIT_COMMIT={value}") ; Ok (()) }
};
}
