// Generated macro for rerun_if_env_changed (function)
macro_rules! Depcrate_outputrerun_if_env_changed {
() => {
// Module: crate::output
// Provides: {"rerun_if_env_changed"}
// Dependencies: {}
# [doc = " The `rerun-if-env-changed` instruction tells Cargo to re-run the build script"] # [doc = " if the value of an environment variable of the given name has changed."] # [doc = ""] # [doc = " Note that the environment variables here are intended for global environment"] # [doc = " variables like `CC` and such, it is not possible to use this for environment"] # [doc = " variables like `TARGET` that [Cargo sets for build scripts][build-env]. The"] # [doc = " environment variables in use are those received by cargo invocations, not"] # [doc = " those received by the executable of the build script."] # [doc = ""] # [doc = " [build-env]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts"] # [track_caller] pub fn rerun_if_env_changed (key : impl AsRef < OsStr >) { let Some (key) = key . as_ref () . to_str () else { panic ! ("cannot emit rerun-if-env-changed: key is not UTF-8") ; } ; if key . contains ('\n') { panic ! ("cannot emit rerun-if-env-changed: key contains newline") ; } emit ("rerun-if-env-changed" , key) ; }
};
}
