// Generated macro for UnwrapInfo (struct)
macro_rules! Depcrate_unwrapUnwrapInfo {
() => {
// Module: crate::unwrap
// Provides: {"UnwrapInfo"}
// Dependencies: {}
# [doc = " Contains information about whether a variable can be unwrapped."] # [derive (Clone , Debug)] struct UnwrapInfo < 'tcx > { # [doc = " The variable that is checked"] local : Local , # [doc = " The if itself"] if_expr : & 'tcx Expr < 'tcx > , # [doc = " The check, like `x.is_ok()`"] check : & 'tcx Expr < 'tcx > , # [doc = " The check's name, like `is_ok`"] check_name : Symbol , # [doc = " The branch where the check takes place, like `if x.is_ok() { .. }`"] branch : & 'tcx Expr < 'tcx > , # [doc = " Whether `is_some()` or `is_ok()` was called (as opposed to `is_err()` or `is_none()`)."] safe_to_unwrap : bool , # [doc = " What kind of unwrappable this is."] kind : UnwrappableKind , # [doc = " If the check is the entire condition (`if x.is_ok()`) or only a part of it (`foo() &&"] # [doc = " x.is_ok()`)"] is_entire_condition : bool , }
};
}
