// Generated macro for macro_447 (macro)
macro_rules! Depcrate_attrsmacro_447 {
() => {
// Module: crate::attrs
// Provides: {"macro_447"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `warn`/`deny`/`forbid` attributes targeting the whole clippy::restriction category."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Restriction lints sometimes are in contrast with other lints or even go against idiomatic rust."] # [doc = " These lints should only be enabled on a lint-by-lint basis and with careful consideration."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #![deny(clippy::restriction)]"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #![deny(clippy::as_conversions)]"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub BLANKET_CLIPPY_RESTRICTION_LINTS , suspicious , "enabling the complete restriction group" }
};
}
