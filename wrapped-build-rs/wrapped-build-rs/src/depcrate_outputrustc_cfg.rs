// Generated macro for rustc_cfg (function)
macro_rules! Depcrate_outputrustc_cfg {
() => {
// Module: crate::output
// Provides: {"rustc_cfg"}
// Dependencies: {}
# [doc = " The `rustc-cfg` instruction tells Cargo to pass the given value to the"] # [doc = " [`--cfg` flag][cfg] to the compiler."] # [doc = ""] # [doc = " This may be used for compile-time"] # [doc = " detection of features to enable conditional compilation."] # [doc = ""] # [doc = " Note that this does not affect Cargo’s dependency resolution. This cannot"] # [doc = " be used to enable an optional dependency, or enable other Cargo features."] # [doc = ""] # [doc = " Be aware that [Cargo features] use the form `feature=\"foo\"`. `cfg` values"] # [doc = " passed with this flag are not restricted to that form, and may provide just"] # [doc = " a single identifier, or any arbitrary key/value pair. For example, emitting"] # [doc = " `rustc_cfg(\"abc\")` will then allow code to use `#[cfg(abc)]` (note the lack"] # [doc = " of `feature=`). Or an arbitrary key/value pair may be used with an `=` symbol"] # [doc = " like `rustc_cfg(r#\"my_component=\"foo\"\"#)`. The key should be a Rust identifier,"] # [doc = " the value should be a string."] # [doc = ""] # [doc = " [cfg]: https://doc.rust-lang.org/rustc/command-line-arguments.html#option-cfg"] # [doc = " [Cargo features]: https://doc.rust-lang.org/cargo/reference/features.html"] # [track_caller] pub fn rustc_cfg (key : & str) { if ! is_ident (key) { panic ! ("cannot emit rustc-cfg: invalid key {key:?}") ; } emit ("rustc-cfg" , key) ; }
};
}
