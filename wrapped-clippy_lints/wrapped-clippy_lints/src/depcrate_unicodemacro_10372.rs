// Generated macro for macro_10372 (macro)
macro_rules! Depcrate_unicodemacro_10372 {
() => {
// Module: crate::unicode
// Provides: {"macro_10372"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for string literals that contain Unicode in a form"] # [doc = " that is not equal to its"] # [doc = " [NFC-recomposition](http://www.unicode.org/reports/tr15/#Norm_Forms)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If such a string is compared to another, the results"] # [doc = " may be surprising."] # [doc = ""] # [doc = " ### Example"] # [doc = " You may not see it, but \"a\u{300}\"\" and \"à\"\" aren't the same string. The"] # [doc = " former when escaped is actually `\"a\\u{300}\"` while the latter is `\"\\u{e0}\"`."] # [clippy :: version = "pre 1.29.0"] pub UNICODE_NOT_NFC , pedantic , "using a Unicode literal not in NFC normal form (see [Unicode tr15](http://www.unicode.org/reports/tr15/) for further information)" }
};
}
