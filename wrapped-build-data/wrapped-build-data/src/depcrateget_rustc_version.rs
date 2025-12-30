// Generated macro for get_rustc_version (function)
macro_rules! Depcrateget_rustc_version {
() => {
// Module: crate
// Provides: {"get_rustc_version"}
// Dependencies: {}
# [doc = " Gets the version of the Rust compiler used to build the build script."] # [doc = ""] # [doc = " Example: `\"rustc 1.53.0-nightly (07e0e2ec2 2021-03-24)\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `rustc` command."] pub fn get_rustc_version () -> Result < String , String > { let rustc_var = get_env ("RUSTC") ? . ok_or ("RUSTC env var is not set") ? ; exec (rustc_var , & ["--version"]) }
};
}
