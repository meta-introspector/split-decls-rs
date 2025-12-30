// Generated macro for macro_1883 (macro)
macro_rules! Depcrate_empty_with_bracketsmacro_1883 {
() => {
// Module: crate::empty_with_brackets
// Provides: {"macro_1883"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds enum variants without fields that are declared with empty brackets."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Empty brackets after a enum variant declaration are redundant and can be omitted,"] # [doc = " and it may be desirable to do so consistently for style."] # [doc = ""] # [doc = " However, removing the brackets also introduces a public constant named after the variant,"] # [doc = " so this is not just a syntactic simplification but an API change, and adding them back"] # [doc = " is a *breaking* API change."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum MyEnum {"] # [doc = "     HasData(u8),"] # [doc = "     HasNoData(),       // redundant parentheses"] # [doc = "     NoneHereEither {}, // redundant braces"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " enum MyEnum {"] # [doc = "     HasData(u8),"] # [doc = "     HasNoData,"] # [doc = "     NoneHereEither,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub EMPTY_ENUM_VARIANTS_WITH_BRACKETS , restriction , "finds enum variants with empty brackets" }
};
}
