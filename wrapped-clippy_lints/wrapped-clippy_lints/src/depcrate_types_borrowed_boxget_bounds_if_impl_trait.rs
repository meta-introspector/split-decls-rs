// Generated macro for get_bounds_if_impl_trait (function)
macro_rules! Depcrate_types_borrowed_boxget_bounds_if_impl_trait {
() => {
// Module: crate::types::borrowed_box
// Provides: {"get_bounds_if_impl_trait"}
// Dependencies: {}
fn get_bounds_if_impl_trait < 'tcx > (cx : & LateContext < 'tcx > , qpath : & QPath < '_ > , id : HirId) -> Option < GenericBounds < 'tcx > > { if let Some (did) = cx . qpath_res (qpath , id) . opt_def_id () && let Some (Node :: GenericParam (generic_param)) = cx . tcx . hir_get_if_local (did) && let GenericParamKind :: Type { synthetic , .. } = generic_param . kind && synthetic && let Some (generics) = cx . tcx . hir_get_generics (id . owner . def_id) && let Some (pred) = generics . bounds_for_param (did . expect_local ()) . next () { Some (pred . bounds) } else { None } }
};
}
