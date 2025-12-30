// Generated macro for macro_7554 (macro)
macro_rules! Depcrate_mut_mutmacro_7554 {
() => {
// Module: crate::mut_mut
// Provides: {"macro_7554"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for instances of `mut mut` references."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Multiple `mut`s don't add anything meaningful to the"] # [doc = " source. This is either a copy'n'paste error, or it shows a fundamental"] # [doc = " misunderstanding of references."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let mut y = 1;"] # [doc = " let x = &mut &mut y;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MUT_MUT , pedantic , "usage of double-mut refs, e.g., `&mut &mut ...`" }
};
}
