// Generated macro for impl_7515 (impl)
macro_rules! Depcrate_missing_docimpl_7515 {
() => {
// Module: crate::missing_doc
// Provides: {"impl_7515"}
// Dependencies: {}
impl MissingDoc { pub fn new (conf : & 'static Conf) -> Self { Self { crate_items_only : conf . missing_docs_in_crate_items , allow_unused : conf . missing_docs_allow_unused , module_depth : 0 , macro_module_depth : 0 , attr_depth : 0 , doc_hidden_depth : 0 , automatically_derived_depth : 0 , in_body : None , require_visibility_at : None , } } fn is_missing_docs (& self , cx : & LateContext < '_ > , def_id : LocalDefId , hir_id : HirId) -> bool { if cx . tcx . sess . opts . test { return false ; } match cx . effective_visibilities . effective_vis (def_id) { None if self . require_visibility_at . is_some () => return false , None if self . crate_items_only && self . module_depth != 0 => return false , Some (vis) if vis . is_public_at_level (Level :: Reexported) => return false , Some (vis) => { if self . crate_items_only { let vis = vis . at_level (Level :: Reachable) ; if ! (vis . is_public () || matches ! (vis , Visibility :: Restricted (id) if id . is_top_level_module ())) { return false ; } } else if let Some (id) = self . require_visibility_at && ! vis . at_level (Level :: Reexported) . is_accessible_from (id , cx . tcx) { return false ; } } , None => { } , } ! cx . tcx . hir_attrs (hir_id) . iter () . any (is_doc_attr) } }
};
}
