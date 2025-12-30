// Generated macro for macro_11360 (macro)
macro_rules! Depcrate_useless_vecmacro_11360 {
() => {
// Module: crate::useless_vec
// Provides: {"macro_11360"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `vec![..]` when using `[..]` would"] # [doc = " be possible."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is less efficient."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(_x: &[u8]) {}"] # [doc = ""] # [doc = " foo(&vec![1, 2]);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn foo(_x: &[u8]) {}"] # [doc = " foo(&[1, 2]);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub USELESS_VEC , perf , "useless `vec!`" }
};
}
