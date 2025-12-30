// Generated macro for macro_4217 (macro)
macro_rules! Depcrate_manual_is_power_of_twomacro_4217 {
() => {
// Module: crate::manual_is_power_of_two
// Provides: {"macro_4217"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions like `x.count_ones() == 1` or `x & (x - 1) == 0`, with x and unsigned integer, which may be manual"] # [doc = " reimplementations of `x.is_power_of_two()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Manual reimplementations of `is_power_of_two` increase code complexity for little benefit."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a: u32 = 4;"] # [doc = " let result = a.count_ones() == 1;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a: u32 = 4;"] # [doc = " let result = a.is_power_of_two();"] # [doc = " ```"] # [clippy :: version = "1.83.0"] pub MANUAL_IS_POWER_OF_TWO , pedantic , "manually reimplementing `is_power_of_two`" }
};
}
