// Generated macro for macro_8624 (macro)
macro_rules! Depcrate_panic_unimplementedmacro_8624 {
() => {
// Module: crate::panic_unimplemented
// Provides: {"macro_8624"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `panic!`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " This macro, or panics in general, may be unwanted in production code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " panic!(\"even with a good reason\");"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub PANIC , restriction , "usage of the `panic!` macro" }
};
}
