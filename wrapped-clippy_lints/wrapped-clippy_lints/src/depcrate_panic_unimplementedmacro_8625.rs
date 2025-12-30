// Generated macro for macro_8625 (macro)
macro_rules! Depcrate_panic_unimplementedmacro_8625 {
() => {
// Module: crate::panic_unimplemented
// Provides: {"macro_8625"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `unimplemented!`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " This macro, or panics in general, may be unwanted in production code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " unimplemented!();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNIMPLEMENTED , restriction , "`unimplemented!` should not be present in production code" }
};
}
