// Generated macro for macro_2757 (macro)
macro_rules! Depcrate_implicit_saturating_submacro_2757 {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"macro_2757"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for implicit saturating subtraction."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Simplicity and readability. Instead we can easily use an builtin function."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let end: u32 = 10;"] # [doc = " # let start: u32 = 5;"] # [doc = " let mut i: u32 = end - start;"] # [doc = ""] # [doc = " if i != 0 {"] # [doc = "     i -= 1;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let end: u32 = 10;"] # [doc = " # let start: u32 = 5;"] # [doc = " let mut i: u32 = end - start;"] # [doc = ""] # [doc = " i = i.saturating_sub(1);"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub IMPLICIT_SATURATING_SUB , style , "Perform saturating subtraction instead of implicitly checking lower bound of data type" }
};
}
