// Generated macro for is_ref_char (function)
macro_rules! Depcrate_methods_single_char_push_stringis_ref_char {
() => {
// Module: crate::methods::single_char_push_string
// Provides: {"is_ref_char"}
// Dependencies: {}
fn is_ref_char (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { if cx . typeck_results () . expr_ty (expr) . is_ref () && let rustc_middle :: ty :: Ref (_ , ty , _) = cx . typeck_results () . expr_ty (expr) . kind () && ty . is_char () { return true ; } false }
};
}
