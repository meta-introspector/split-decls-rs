// Generated macro for macro_2738 (macro)
macro_rules! Depcrate_implicit_saturating_addmacro_2738 {
() => {
// Module: crate::implicit_saturating_add
// Provides: {"macro_2738"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for implicit saturating addition."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The built-in function is more readable and may be faster."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = "let mut u:u32 = 7000;"] # [doc = ""] # [doc = " if u != u32::MAX {"] # [doc = "     u += 1;"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = "let mut u:u32 = 7000;"] # [doc = ""] # [doc = " u = u.saturating_add(1);"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub IMPLICIT_SATURATING_ADD , style , "Perform saturating addition instead of implicitly checking max bound of data type" }
};
}
