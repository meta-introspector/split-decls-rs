// Generated macro for macro_446 (macro)
macro_rules! Depcrate_attrsmacro_446 {
() => {
// Module: crate::attrs
// Provides: {"macro_446"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[deprecated]` annotations with a `since`"] # [doc = " field that is not a valid semantic version. Also allows \"TBD\" to signal"] # [doc = " future deprecation."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " For checking the version of the deprecation, it must be"] # [doc = " a valid semver. Failing that, the contained information is useless."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[deprecated(since = \"forever\")]"] # [doc = " fn something_else() { /* ... */ }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DEPRECATED_SEMVER , correctness , "use of `#[deprecated(since = \"x\")]` where x is not semver" }
};
}
