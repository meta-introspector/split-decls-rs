// Generated macro for rerun_if_git_commit_or_branch_changed (function)
macro_rules! Depcratererun_if_git_commit_or_branch_changed {
() => {
// Module: crate
// Provides: {"rerun_if_git_commit_or_branch_changed"}
// Dependencies: {}
# [doc = " Tells Cargo to re-run the build if the git commit or branch changed."] # [doc = ""] # [doc = " NOTE: By default, Cargo re-runs `build.rs` when any file in the directory changes."] # [doc = " But if you use any `rerun-if-changed` rule, then it removes that default and uses only the rules"] # [doc = " you specify.  Only call this function if you are already using `rerun-if-changed` rules."] # [doc = " See [cargo::rerun-if-changed=PATH](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `git` command."] pub fn rerun_if_git_commit_or_branch_changed () -> Result < () , String > { let git_dirpath = exec ("git" , & ["rev-parse" , "--git-dir"]) ? ; println ! ("cargo::rerun-if-changed={git_dirpath}/index") ; println ! ("cargo::rerun-if-changed={git_dirpath}/logs/HEAD") ; Ok (()) }
};
}
