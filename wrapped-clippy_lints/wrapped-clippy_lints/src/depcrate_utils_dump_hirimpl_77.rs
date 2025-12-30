// Generated macro for impl_77 (impl)
macro_rules! Depcrate_utils_dump_hirimpl_77 {
() => {
// Module: crate::utils::dump_hir
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DumpHir { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < '_ >) { if has_attr (cx , item . hir_id ()) { println ! ("{item:#?}") ; } } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { if has_attr (cx , expr . hir_id) { println ! ("{expr:#?}") ; } } fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx hir :: Stmt < '_ >) { match stmt . kind { hir :: StmtKind :: Expr (e) | hir :: StmtKind :: Semi (e) if has_attr (cx , e . hir_id) => return , _ => { } , } if has_attr (cx , stmt . hir_id) { println ! ("{stmt:#?}") ; } } fn check_trait_item (& mut self , cx : & LateContext < '_ > , item : & TraitItem < '_ >) { if has_attr (cx , item . hir_id ()) { println ! ("{item:#?}") ; } } fn check_impl_item (& mut self , cx : & LateContext < '_ > , item : & hir :: ImplItem < '_ >) { if has_attr (cx , item . hir_id ()) { println ! ("{item:#?}") ; } } }
};
}
