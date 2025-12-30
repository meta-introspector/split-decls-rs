// Generated macro for get_target_platform (function)
macro_rules! Depcrateget_target_platform {
() => {
// Module: crate
// Provides: {"get_target_platform"}
// Dependencies: {}
# [doc = " Gets the Rust target platform string from the TARGET env var"] # [doc = " [set by cargo for build scripts](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)."] # [doc = " See [`set_TARGET_PLATFORM`]."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns `Err` when the `TARGET` env var is not set or is not valid UTF-8."] pub fn get_target_platform () -> Result < String , String > { get_env ("TARGET") ? . ok_or_else (| | String :: from ("TARGET env var is empty")) }
};
}
