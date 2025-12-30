// Generated macro for set_TARGET_PLATFORM (function)
macro_rules! Depcrateset_TARGET_PLATFORM {
() => {
// Module: crate
// Provides: {"set_TARGET_PLATFORM"}
// Dependencies: {}
# [doc = " Sets the `TARGET_PLATFORM` env variable to the Rust target triple."] # [doc = " See \"Target Triple\" in"] # [doc = " [The Cargo Book - Glossary](https://doc.rust-lang.org/cargo/appendix/glossary.html#target)."] # [doc = ""] # [doc = " Gets the string from the TARGET env var"] # [doc = " [set by cargo for build scripts](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)."] # [doc = ""] # [doc = " Examples:"] # [doc = " - `x86_64-unknown-linux-gnu` (Linux on Intel)"] # [doc = " - `x86_64-apple-darwin` (macOS on Intel)"] # [doc = " - `aarch64-unknown-linux-gnu` (Linux on ARM)"] # [doc = " - `aarch64-apple-darwin` (macOS on Apple Silicon)"] # [doc = " - See <https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools>"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns `Err` when the `TARGET` env var is not set."] pub fn set_TARGET_PLATFORM () -> Result < () , String > { let value = get_target_platform () ? ; println ! ("cargo:rustc-env=TARGET_PLATFORM={value}") ; Ok (()) }
};
}
