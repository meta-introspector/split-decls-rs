// Generated macro for check_invalid_clippy_version_attribute (function)
macro_rules! Depcrate_lint_without_lint_passcheck_invalid_clippy_version_attribute {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"check_invalid_clippy_version_attribute"}
// Dependencies: {}
fn check_invalid_clippy_version_attribute (cx : & LateContext < '_ > , item : & '_ Item < '_ >) { if let Some (value) = extract_clippy_version_value (cx , item) { if value . as_str () == "pre 1.29.0" { return ; } if rustc_attr_parsing :: parse_version (value) . is_none () { span_lint_and_help (cx , INVALID_CLIPPY_VERSION_ATTRIBUTE , item . span , "this item has an invalid `clippy::version` attribute" , None , "please use a valid semantic version, see `doc/adding_lints.md`" ,) ; } } else { span_lint_and_help (cx , MISSING_CLIPPY_VERSION_ATTRIBUTE , item . span , "this lint is missing the `clippy::version` attribute or version value" , None , "please use a `clippy::version` attribute, see `doc/adding_lints.md`" ,) ; } }
};
}
