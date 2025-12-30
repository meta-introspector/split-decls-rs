// Generated macro for macro_1909 (macro)
macro_rules! Depcrate_empty_with_bracketsmacro_1909 {
() => {
// Module: crate::empty_with_brackets
// Provides: {"macro_1909"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds structs without fields (a so-called \"empty struct\") that are declared with brackets."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Empty brackets after a struct declaration can be omitted,"] # [doc = " and it may be desirable to do so consistently for style."] # [doc = ""] # [doc = " However, removing the brackets also introduces a public constant named after the struct,"] # [doc = " so this is not just a syntactic simplification but an API change, and adding them back"] # [doc = " is a *breaking* API change."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Cookie {}"] # [doc = " struct Biscuit();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct Cookie;"] # [doc = " struct Biscuit;"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub EMPTY_STRUCTS_WITH_BRACKETS , restriction , "finds struct declarations with empty brackets" }
};
}
