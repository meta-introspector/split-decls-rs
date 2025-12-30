// Generated macro for macro_2325 (macro)
macro_rules! Depcrate_formattingmacro_2325 {
() => {
// Module: crate::formatting
// Provides: {"macro_2325"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for possible missing comma in an array. It lints if"] # [doc = " an array element is a binary operator expression and it lies on two lines."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This could lead to unexpected results."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let a = &["] # [doc = "     -1, -2, -3 // <= no comma here"] # [doc = "     -4, -5, -6"] # [doc = " ];"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub POSSIBLE_MISSING_COMMA , correctness , "possible missing comma in array" }
};
}
