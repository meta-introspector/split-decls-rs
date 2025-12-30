// Generated macro for set_RUSTC_VERSION (function)
macro_rules! Depcrateset_RUSTC_VERSION {
() => {
// Module: crate
// Provides: {"set_RUSTC_VERSION"}
// Dependencies: {}
# [doc = " Sets the `RUSTC_VERSION` env variable to the output of `rustc --version`."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Example value: `\"rustc 1.53.0-nightly (07e0e2ec2 2021-03-24)\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `rustc` command."] pub fn set_RUSTC_VERSION () -> Result < () , String > { let value = get_rustc_version () ? ; println ! ("cargo:rustc-env=RUSTC_VERSION={value}") ; Ok (()) }
};
}
