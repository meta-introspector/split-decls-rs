// Generated macro for impl_462 (impl)
macro_rules! Depcrate_attrsimpl_462 {
() => {
// Module: crate::attrs
// Provides: {"impl_462"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Attributes { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { let attrs = cx . tcx . hir_attrs (item . hir_id ()) ; if let ItemKind :: Fn { ident , .. } = item . kind && is_relevant_item (cx , item) { inline_always :: check (cx , item . span , ident . name , attrs) ; } repr_attributes :: check (cx , item . span , attrs , self . msrv) ; } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx ImplItem < '_ >) { if is_relevant_impl (cx , item) { inline_always :: check (cx , item . span , item . ident . name , cx . tcx . hir_attrs (item . hir_id ())) ; } } fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx TraitItem < '_ >) { if is_relevant_trait (cx , item) { inline_always :: check (cx , item . span , item . ident . name , cx . tcx . hir_attrs (item . hir_id ())) ; } } }
};
}
