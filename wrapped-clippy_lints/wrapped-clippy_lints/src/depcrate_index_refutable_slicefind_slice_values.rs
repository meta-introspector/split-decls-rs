// Generated macro for find_slice_values (function)
macro_rules! Depcrate_index_refutable_slicefind_slice_values {
() => {
// Module: crate::index_refutable_slice
// Provides: {"find_slice_values"}
// Dependencies: {}
fn find_slice_values (cx : & LateContext < '_ > , pat : & hir :: Pat < '_ >) -> FxIndexMap < HirId , SliceLintInformation > { let mut removed_pat : FxHashSet < HirId > = FxHashSet :: default () ; let mut slices : FxIndexMap < HirId , SliceLintInformation > = FxIndexMap :: default () ; pat . walk_always (| pat | { if let hir :: PatKind :: Binding (hir :: BindingMode (by_ref , hir :: Mutability :: Not) , value_hir_id , ident , sub_pat) = pat . kind && ! matches ! (by_ref , hir :: ByRef :: Yes (_ , hir :: Mutability :: Mut)) { if removed_pat . contains (& value_hir_id) { return ; } if sub_pat . is_some () { removed_pat . insert (value_hir_id) ; slices . swap_remove (& value_hir_id) ; return ; } let bound_ty = cx . typeck_results () . node_type (pat . hir_id) ; if let Some (inner_ty) = bound_ty . peel_refs () . builtin_index () { let src_is_ref = bound_ty . is_ref () && by_ref == hir :: ByRef :: No ; let needs_ref = ! (src_is_ref || is_copy (cx , inner_ty)) ; let slice_info = slices . entry (value_hir_id) . or_insert_with (| | SliceLintInformation :: new (ident , needs_ref)) ; slice_info . pattern_spans . push (pat . span) ; } } }) ; slices }
};
}
