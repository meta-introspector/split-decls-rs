// Generated macro for macro_8547 (macro)
macro_rules! Depcrate_operatorsmacro_8547 {
() => {
// Module: crate::operators
// Provides: {"macro_8547"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual implementation of `midpoint`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `(x + y) / 2` might cause an overflow on the intermediate"] # [doc = " addition result."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let a: u32 = 0;"] # [doc = " let c = (a + 10) / 2;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let a: u32 = 0;"] # [doc = " let c = u32::midpoint(a, 10);"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub MANUAL_MIDPOINT , pedantic , "manual implementation of `midpoint` which can overflow" }
};
}
