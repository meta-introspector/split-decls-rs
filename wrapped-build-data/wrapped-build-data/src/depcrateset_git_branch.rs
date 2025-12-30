// Generated macro for set_GIT_BRANCH (function)
macro_rules! Depcrateset_GIT_BRANCH {
() => {
// Module: crate
// Provides: {"set_GIT_BRANCH"}
// Dependencies: {}
# [doc = " Sets the `GIT_BRANCH` env variable."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Example value: `\"release\"`"] # [doc = ""] # [doc = " Related: [`rerun_if_git_commit_or_branch_changed`]"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn set_GIT_BRANCH () -> Result < () , String > { let value = get_git_branch () ? ; println ! ("cargo:rustc-env=GIT_BRANCH={value}") ; Ok (()) }
};
}
