// Generated macro for set_GIT_COMMIT_SHORT (function)
macro_rules! Depcrateset_GIT_COMMIT_SHORT {
() => {
// Module: crate
// Provides: {"set_GIT_COMMIT_SHORT"}
// Dependencies: {}
# [doc = " Sets the `GIT_COMMIT_SHORT` env variable."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Example value: `\"a5547bf\"`"] # [doc = ""] # [doc = " Related: [`rerun_if_git_commit_or_branch_changed`]"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn set_GIT_COMMIT_SHORT () -> Result < () , String > { let value = get_git_commit_short () ? ; println ! ("cargo:rustc-env=GIT_COMMIT_SHORT={value}") ; Ok (()) }
};
}
