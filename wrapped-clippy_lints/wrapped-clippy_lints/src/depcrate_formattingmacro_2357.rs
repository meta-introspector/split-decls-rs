// Generated macro for macro_2357 (macro)
macro_rules! Depcrate_formattingmacro_2357 {
() => {
// Module: crate::formatting
// Provides: {"macro_2357"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for an `if` expression followed by either a block or another `if` that"] # [doc = " looks like it should have an `else` between them."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably some refactoring remnant, even if the code is correct, it"] # [doc = " might look confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " if foo {"] # [doc = " } { // looks like an `else` is missing here"] # [doc = " }"] # [doc = ""] # [doc = " if foo {"] # [doc = " } if bar { // looks like an `else` is missing here"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.91.0"] pub POSSIBLE_MISSING_ELSE , suspicious , "possibly missing `else`" }
};
}
