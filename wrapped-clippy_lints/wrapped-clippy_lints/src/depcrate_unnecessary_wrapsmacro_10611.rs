// Generated macro for macro_10611 (macro)
macro_rules! Depcrate_unnecessary_wrapsmacro_10611 {
() => {
// Module: crate::unnecessary_wraps
// Provides: {"macro_10611"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for private functions that only return `Ok` or `Some`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is not meaningful to wrap values when no `None` or `Err` is returned."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " There can be false positives if the function signature is designed to"] # [doc = " fit some external requirement."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn get_cool_number(a: bool, b: bool) -> Option<i32> {"] # [doc = "     if a && b {"] # [doc = "         return Some(50);"] # [doc = "     }"] # [doc = "     if a {"] # [doc = "         Some(0)"] # [doc = "     } else {"] # [doc = "         Some(10)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn get_cool_number(a: bool, b: bool) -> i32 {"] # [doc = "     if a && b {"] # [doc = "         return 50;"] # [doc = "     }"] # [doc = "     if a {"] # [doc = "         0"] # [doc = "     } else {"] # [doc = "         10"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.50.0"] pub UNNECESSARY_WRAPS , pedantic , "functions that only return `Ok` or `Some`" }
};
}
