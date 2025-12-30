// Generated macro for is_local_vec_expn (function)
macro_rules! Depcrate_box_defaultis_local_vec_expn {
() => {
// Module: crate::box_default
// Provides: {"is_local_vec_expn"}
// Dependencies: {}
fn is_local_vec_expn (cx : & LateContext < '_ > , expr : & Expr < '_ > , ref_expr : & Expr < '_ >) -> bool { macro_backtrace (expr . span) . next () . is_some_and (| call | cx . tcx . is_diagnostic_item (sym :: vec_macro , call . def_id) && call . span . eq_ctxt (ref_expr . span)) }
};
}
