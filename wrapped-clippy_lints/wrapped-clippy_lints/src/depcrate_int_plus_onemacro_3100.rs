// Generated macro for macro_3100 (macro)
macro_rules! Depcrate_int_plus_onemacro_3100 {
() => {
// Module: crate::int_plus_one
// Provides: {"macro_3100"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `x >= y + 1` or `x - 1 >= y` (and `<=`) in a block"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability -- better to use `> y` instead of `>= y + 1`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " # let y = 1;"] # [doc = " if x >= y + 1 {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " # let y = 1;"] # [doc = " if x > y {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INT_PLUS_ONE , complexity , "instead of using `x >= y + 1`, use `x > y`" }
};
}
