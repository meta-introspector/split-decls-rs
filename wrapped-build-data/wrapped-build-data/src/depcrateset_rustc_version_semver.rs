// Generated macro for set_RUSTC_VERSION_SEMVER (function)
macro_rules! Depcrateset_RUSTC_VERSION_SEMVER {
() => {
// Module: crate
// Provides: {"set_RUSTC_VERSION_SEMVER"}
// Dependencies: {}
# [doc = " Sets the `RUSTC_VERSION_SEMVER` to the dotted version number of the `rustc`"] # [doc = " used by the current build."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Example value: `\"1.53.0\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `rustc` command."] pub fn set_RUSTC_VERSION_SEMVER () -> Result < () , String > { let version = get_rustc_version () ? ; let semver = parse_rustc_semver (version) ? ; println ! ("cargo:rustc-env=RUSTC_VERSION_SEMVER={semver}") ; Ok (()) }
};
}
