// Generated macro for macro_3369 (macro)
macro_rules! Depcrate_let_if_seqmacro_3369 {
() => {
// Module: crate::let_if_seq
// Provides: {"macro_3369"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for variable declarations immediately followed by a"] # [doc = " conditional affectation."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is not idiomatic Rust."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let foo;"] # [doc = ""] # [doc = " if bar() {"] # [doc = "     foo = 42;"] # [doc = " } else {"] # [doc = "     foo = 0;"] # [doc = " }"] # [doc = ""] # [doc = " let mut baz = None;"] # [doc = ""] # [doc = " if bar() {"] # [doc = "     baz = Some(42);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " should be written"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " let foo = if bar() {"] # [doc = "     42"] # [doc = " } else {"] # [doc = "     0"] # [doc = " };"] # [doc = ""] # [doc = " let baz = if bar() {"] # [doc = "     Some(42)"] # [doc = " } else {"] # [doc = "     None"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub USELESS_LET_IF_SEQ , nursery , "unidiomatic `let mut` declaration followed by initialization in `if`" }
};
}
