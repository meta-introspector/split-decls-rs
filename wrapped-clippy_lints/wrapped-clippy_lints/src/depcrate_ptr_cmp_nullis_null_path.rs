// Generated macro for is_null_path (function)
macro_rules! Depcrate_ptr_cmp_nullis_null_path {
() => {
// Module: crate::ptr::cmp_null
// Provides: {"is_null_path"}
// Dependencies: {}
fn is_null_path (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let ExprKind :: Call (pathexp , []) = expr . kind { matches ! (pathexp . basic_res () . opt_diag_name (cx) , Some (sym :: ptr_null | sym :: ptr_null_mut)) } else { false } }
};
}
