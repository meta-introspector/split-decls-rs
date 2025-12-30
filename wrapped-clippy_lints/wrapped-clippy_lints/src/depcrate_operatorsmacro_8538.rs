// Generated macro for macro_8538 (macro)
macro_rules! Depcrate_operatorsmacro_8538 {
() => {
// Module: crate::operators
// Provides: {"macro_8538"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for identity operations, e.g., `x + 0`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This code can be removed without changing the"] # [doc = " meaning. So it just obscures what's going on. Delete it mercilessly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " x / 1 + 0 * 1 - 0 | 0;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub IDENTITY_OP , complexity , "using identity operations, e.g., `x + 0` or `y / 1`" }
};
}
