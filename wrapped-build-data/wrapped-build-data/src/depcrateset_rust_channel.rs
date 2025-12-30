// Generated macro for set_RUST_CHANNEL (function)
macro_rules! Depcrateset_RUST_CHANNEL {
() => {
// Module: crate
// Provides: {"set_RUST_CHANNEL"}
// Dependencies: {}
# [doc = " Sets the `RUST_CHANNEL` env variable to Rust channel used by the current build."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Possible values:"] # [doc = " - `\"stable\"`"] # [doc = " - `\"beta\"`"] # [doc = " - `\"nightly\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `rustc` command."] pub fn set_RUST_CHANNEL () -> Result < () , String > { let version = get_rustc_version () ? ; let channel = parse_rustc_channel (version) ? ; println ! ("cargo:rustc-env=RUST_CHANNEL={channel}") ; Ok (()) }
};
}
