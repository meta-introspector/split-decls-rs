// Generated macro for macro_2860 (macro)
macro_rules! Depcrate_implicit_saturating_submacro_2860 {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"macro_2860"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for comparisons between integers, followed by subtracting the greater value from the"] # [doc = " lower one."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This could result in an underflow and is most likely not what the user wants. If this was"] # [doc = " intended to be a saturated subtraction, consider using the `saturating_sub` method directly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = 12u32;"] # [doc = " let b = 13u32;"] # [doc = ""] # [doc = " let result = if a > b { b - a } else { 0 };"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = 12u32;"] # [doc = " let b = 13u32;"] # [doc = ""] # [doc = " let result = a.saturating_sub(b);"] # [doc = " ```"] # [clippy :: version = "1.83.0"] pub INVERTED_SATURATING_SUB , correctness , "Check if a variable is smaller than another one and still subtract from it even if smaller" }
};
}
