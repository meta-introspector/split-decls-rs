// Generated macro for macro_7626 (macro)
macro_rules! Depcrate_needless_boolmacro_7626 {
() => {
// Module: crate::needless_bool
// Provides: {"macro_7626"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions of the form `if c { true } else {"] # [doc = " false }` (or vice versa) and suggests using the condition directly."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant code."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Maybe false positives: Sometimes, the two branches are"] # [doc = " painstakingly documented (which we, of course, do not detect), so they *may*"] # [doc = " have some value. Even then, the documentation can be rewritten to match the"] # [doc = " shorter code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = true;"] # [doc = " if x {"] # [doc = "     false"] # [doc = " } else {"] # [doc = "     true"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = true;"] # [doc = " !x"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_BOOL , complexity , "if-statements with plain booleans in the then- and else-clause, e.g., `if p { true } else { false }`" }
};
}
