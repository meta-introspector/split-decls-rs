// Generated macro for cargo_encoded_rustflags (function)
macro_rules! Depcrate_inputcargo_encoded_rustflags {
() => {
// Module: crate::input
// Provides: {"cargo_encoded_rustflags"}
// Dependencies: {}
# [doc = " Extra flags that Cargo invokes rustc with. See [`build.rustflags`]."] # [doc = ""] # [doc = " [`build.rustflags`]: https://doc.rust-lang.org/stable/cargo/reference/config.html#buildrustflags"] # [track_caller] pub fn cargo_encoded_rustflags () -> Vec < String > { to_strings (var_or_panic ("CARGO_ENCODED_RUSTFLAGS") , '\x1f') }
};
}
