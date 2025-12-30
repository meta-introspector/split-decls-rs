// Generated macro for impl_62 (impl)
macro_rules! Depcrate_utils_authorimpl_62 {
() => {
// Module: crate::utils::author
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Author { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < '_ >) { check_item (cx , item . hir_id ()) ; } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: ImplItem < '_ >) { check_item (cx , item . hir_id ()) ; } fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: TraitItem < '_ >) { check_item (cx , item . hir_id ()) ; } fn check_arm (& mut self , cx : & LateContext < 'tcx > , arm : & 'tcx hir :: Arm < '_ >) { check_node (cx , arm . hir_id , | v | { v . arm (& v . bind ("arm" , arm)) ; }) ; } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { check_node (cx , expr . hir_id , | v | { v . expr (& v . bind ("expr" , expr)) ; }) ; } fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx hir :: Stmt < '_ >) { match stmt . kind { StmtKind :: Expr (e) | StmtKind :: Semi (e) if has_attr (cx , e . hir_id) => return , _ => { } , } check_node (cx , stmt . hir_id , | v | { v . stmt (& v . bind ("stmt" , stmt)) ; }) ; } }
};
}
