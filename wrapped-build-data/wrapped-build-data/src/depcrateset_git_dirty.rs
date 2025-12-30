// Generated macro for set_GIT_DIRTY (function)
macro_rules! Depcrateset_GIT_DIRTY {
() => {
// Module: crate
// Provides: {"set_GIT_DIRTY"}
// Dependencies: {}
# [doc = " Sets the `GIT_DIRTY` env variable."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Sets the variable to `\"true\"` if the git repository contains uncommitted"] # [doc = " changes.  Otherwise, sets it to `\"false\"`."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn set_GIT_DIRTY () -> Result < () , String > { let value = get_git_dirty () ? ; println ! ("cargo:rustc-env=GIT_DIRTY={value}") ; Ok (()) }
};
}
