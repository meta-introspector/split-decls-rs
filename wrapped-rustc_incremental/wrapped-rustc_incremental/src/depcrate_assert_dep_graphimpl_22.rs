// Generated macro for impl_22 (impl)
macro_rules! Depcrate_assert_dep_graphimpl_22 {
() => {
// Module: crate::assert_dep_graph
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for IfThisChanged < 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , item : & 'tcx hir :: Item < 'tcx >) { self . process_attrs (item . owner_id . def_id) ; intravisit :: walk_item (self , item) ; } fn visit_trait_item (& mut self , trait_item : & 'tcx hir :: TraitItem < 'tcx >) { self . process_attrs (trait_item . owner_id . def_id) ; intravisit :: walk_trait_item (self , trait_item) ; } fn visit_impl_item (& mut self , impl_item : & 'tcx hir :: ImplItem < 'tcx >) { self . process_attrs (impl_item . owner_id . def_id) ; intravisit :: walk_impl_item (self , impl_item) ; } fn visit_field_def (& mut self , s : & 'tcx hir :: FieldDef < 'tcx >) { self . process_attrs (s . def_id) ; intravisit :: walk_field_def (self , s) ; } }
};
}
