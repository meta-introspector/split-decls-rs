// Generated macro for macro_7940 (macro)
macro_rules! Depcrate_needless_parens_on_range_literalsmacro_7940 {
() => {
// Module: crate::needless_parens_on_range_literals
// Provides: {"macro_7940"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " The lint checks for parenthesis on literals in range statements that are"] # [doc = " superfluous."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Having superfluous parenthesis makes the code less readable"] # [doc = " overhead when reading."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " for i in (0)..10 {"] # [doc = "   println!(\"{i}\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " for i in 0..10 {"] # [doc = "   println!(\"{i}\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub NEEDLESS_PARENS_ON_RANGE_LITERALS , style , "needless parenthesis on range literals can be removed" }
};
}
