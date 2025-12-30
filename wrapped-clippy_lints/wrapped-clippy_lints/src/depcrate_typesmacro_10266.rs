// Generated macro for macro_10266 (macro)
macro_rules! Depcrate_typesmacro_10266 {
() => {
// Module: crate::types
// Provides: {"macro_10266"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Option<Option<_>>` in function signatures and type"] # [doc = " definitions"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Option<_>` represents an optional value. `Option<Option<_>>`"] # [doc = " represents an optional value which itself wraps an optional. This is logically the"] # [doc = " same thing as an optional value but has an unneeded extra level of wrapping."] # [doc = ""] # [doc = " If you have a case where `Some(Some(_))`, `Some(None)` and `None` are distinct cases,"] # [doc = " consider a custom `enum` instead, with clear names for each case."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn get_data() -> Option<Option<u32>> {"] # [doc = "     None"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " pub enum Contents {"] # [doc = "     Data(Vec<u8>), // Was Some(Some(Vec<u8>))"] # [doc = "     NotYetFetched, // Was Some(None)"] # [doc = "     None,          // Was None"] # [doc = " }"] # [doc = ""] # [doc = " fn get_data() -> Contents {"] # [doc = "     Contents::None"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OPTION_OPTION , pedantic , "usage of `Option<Option<T>>`" }
};
}
