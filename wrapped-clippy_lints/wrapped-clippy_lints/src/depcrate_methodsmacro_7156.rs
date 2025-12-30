// Generated macro for macro_7156 (macro)
macro_rules! Depcrate_methodsmacro_7156 {
() => {
// Module: crate::methods
// Provides: {"macro_7156"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `offset(_)`, `wrapping_`{`add`, `sub`}, etc. on raw pointers to"] # [doc = " zero-sized types"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is a no-op, and likely unintended"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " unsafe { (&() as *const ()).offset(1) };"] # [doc = " ```"] # [clippy :: version = "1.41.0"] pub ZST_OFFSET , correctness , "Check for offset calculations on raw pointers to zero-sized types" }
};
}
