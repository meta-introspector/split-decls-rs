// Generated macro for check_cargo_clippy_attr (function)
macro_rules! Depcrate_attrs_deprecated_cfg_attrcheck_cargo_clippy_attr {
() => {
// Module: crate::attrs::deprecated_cfg_attr
// Provides: {"check_cargo_clippy_attr"}
// Dependencies: {}
fn check_cargo_clippy_attr (cx : & EarlyContext < '_ > , item : & rustc_ast :: MetaItem) { if item . has_name (sym :: feature) && item . value_str () == Some (sym :: cargo_clippy) { span_lint_and_sugg (cx , DEPRECATED_CLIPPY_CFG_ATTR , item . span , "`feature = \"cargo-clippy\"` was replaced by `clippy`" , "replace with" , "clippy" . to_string () , Applicability :: MachineApplicable ,) ; } }
};
}
