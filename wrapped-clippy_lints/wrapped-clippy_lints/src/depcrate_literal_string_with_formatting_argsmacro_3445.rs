// Generated macro for macro_3445 (macro)
macro_rules! Depcrate_literal_string_with_formatting_argsmacro_3445 {
() => {
// Module: crate::literal_string_with_formatting_args
// Provides: {"macro_3445"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if string literals have formatting arguments outside of macros"] # [doc = " using them (like `format!`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It will likely not generate the expected content."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: Option<usize> = None;"] # [doc = " let y = \"hello\";"] # [doc = " x.expect(\"{y:?}\");"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x: Option<usize> = None;"] # [doc = " let y = \"hello\";"] # [doc = " x.expect(&format!(\"{y:?}\"));"] # [doc = " ```"] # [clippy :: version = "1.85.0"] pub LITERAL_STRING_WITH_FORMATTING_ARGS , nursery , "Checks if string literals have formatting arguments" }
};
}
