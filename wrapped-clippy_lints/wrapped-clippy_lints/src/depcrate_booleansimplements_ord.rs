// Generated macro for implements_ord (function)
macro_rules! Depcrate_booleansimplements_ord {
() => {
// Module: crate::booleans
// Provides: {"implements_ord"}
// Dependencies: {}
fn implements_ord (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let ty = cx . typeck_results () . expr_ty (expr) ; cx . tcx . get_diagnostic_item (sym :: Ord) . is_some_and (| id | implements_trait (cx , ty , id , & [])) }
};
}
