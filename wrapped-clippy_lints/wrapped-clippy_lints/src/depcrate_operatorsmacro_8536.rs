// Generated macro for macro_8536 (macro)
macro_rules! Depcrate_operatorsmacro_8536 {
() => {
// Module: crate::operators
// Provides: {"macro_8536"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for erasing operations, e.g., `x * 0`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The whole expression can be replaced by zero."] # [doc = " This is most likely not the intended outcome and should probably be"] # [doc = " corrected"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 1;"] # [doc = " 0 / x;"] # [doc = " 0 * x;"] # [doc = " x & 0;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ERASING_OP , correctness , "using erasing operations, e.g., `x * 0` or `y & 0`" }
};
}
