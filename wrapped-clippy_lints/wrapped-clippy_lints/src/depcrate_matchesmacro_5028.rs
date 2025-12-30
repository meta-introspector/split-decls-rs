// Generated macro for macro_5028 (macro)
macro_rules! Depcrate_matchesmacro_5028 {
() => {
// Module: crate::matches
// Provides: {"macro_5028"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for useless match that binds to only one value."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability and needless complexity."] # [doc = ""] # [doc = " ### Known problems"] # [doc = "  Suggested replacements may be incorrect when `match`"] # [doc = " is actually binding temporary value, bringing a 'dropped while borrowed' error."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let a = 1;"] # [doc = " # let b = 2;"] # [doc = " match (a, b) {"] # [doc = "     (c, d) => {"] # [doc = "         // useless match"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let a = 1;"] # [doc = " # let b = 2;"] # [doc = " let (c, d) = (a, b);"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub MATCH_SINGLE_BINDING , complexity , "a match with a single binding instead of using `let` statement" }
};
}
