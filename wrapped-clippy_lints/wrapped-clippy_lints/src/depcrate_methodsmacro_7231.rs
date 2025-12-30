// Generated macro for macro_7231 (macro)
macro_rules! Depcrate_methodsmacro_7231 {
() => {
// Module: crate::methods
// Provides: {"macro_7231"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary calls to `min()` or `max()` in the following cases"] # [doc = " - Either both side is constant"] # [doc = " - One side is clearly larger than the other, like i32::MIN and an i32 variable"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " In the aforementioned cases it is not necessary to call `min()` or `max()`"] # [doc = " to compare values, it may even cause confusion."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = 0.min(7_u32);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = 0;"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub UNNECESSARY_MIN_OR_MAX , complexity , "using 'min()/max()' when there is no need for it" }
};
}
