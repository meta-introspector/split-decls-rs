// Generated macro for no_debug_rebuilds (function)
macro_rules! Depcrateno_debug_rebuilds {
() => {
// Module: crate
// Provides: {"no_debug_rebuilds"}
// Dependencies: {}
# [doc = " Tells cargo not to rebuild `build.rs` during debug builds when other files"] # [doc = " change."] # [doc = ""] # [doc = " This speeds up development builds."] # [doc = ""] # [doc = " Reads the PROFILE env var"] # [doc = " [set by cargo for build scripts](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when the PROFILE env var is not set."] pub fn no_debug_rebuilds () -> Result < () , String > { let profile = get_env ("PROFILE") ? . ok_or ("PROFILE env var not set") ? ; if & profile == "debug" { println ! ("cargo:rerun-if-env-changed=PROFILE") ; } Ok (()) }
};
}
