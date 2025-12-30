// Generated macro for impl_7737 (impl)
macro_rules! Depcrate_mutex_atomicimpl_7737 {
() => {
// Module: crate::mutex_atomic
// Provides: {"impl_7737"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Mutex { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if ! item . span . from_expansion () && let ItemKind :: Static (_ , _ , ty , body_id) = item . kind { let body = cx . tcx . hir_body (body_id) ; let mid_ty = ty_from_hir_ty (cx , ty) ; check_expr (cx , body . value . peel_blocks () , & TypeAscriptionKind :: Required (ty) , mid_ty) ; } } fn check_local (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx LetStmt < '_ >) { if ! stmt . span . from_expansion () && let Some (init) = stmt . init { let mid_ty = cx . typeck_results () . expr_ty (init) ; check_expr (cx , init . peel_blocks () , & TypeAscriptionKind :: Optional (stmt . ty) , mid_ty) ; } } }
};
}
