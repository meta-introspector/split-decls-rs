// Generated macro for is_null_path (function)
macro_rules! Depcrate_ptris_null_path {
() => {
// Module: crate::ptr
// Provides: {"is_null_path"}
// Dependencies: {}
fn is_null_path (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let ExprKind :: Call (pathexp , []) = expr . kind { path_def_id (cx , pathexp) . is_some_and (| id | matches ! (cx . tcx . get_diagnostic_name (id) , Some (sym :: ptr_null | sym :: ptr_null_mut))) } else { false } }
};
}
