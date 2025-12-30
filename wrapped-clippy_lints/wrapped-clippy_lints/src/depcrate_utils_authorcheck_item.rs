// Generated macro for check_item (function)
macro_rules! Depcrate_utils_authorcheck_item {
() => {
// Module: crate::utils::author
// Provides: {"check_item"}
// Dependencies: {}
fn check_item (cx : & LateContext < '_ > , hir_id : HirId) { if let Some (body) = cx . tcx . hir_maybe_body_owned_by (hir_id . expect_owner () . def_id) { check_node (cx , hir_id , | v | { v . expr (& v . bind ("expr" , body . value)) ; }) ; } }
};
}
