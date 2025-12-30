// Generated macro for extract_future_output (function)
macro_rules! Depcrate_len_zeroextract_future_output {
() => {
// Module: crate::len_zero
// Provides: {"extract_future_output"}
// Dependencies: {}
fn extract_future_output < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < & 'tcx PathSegment < 'tcx > > { if let ty :: Alias (_ , alias_ty) = ty . kind () && let Some (Node :: OpaqueTy (opaque)) = cx . tcx . hir_get_if_local (alias_ty . def_id) && let OpaqueTyOrigin :: AsyncFn { .. } = opaque . origin && let [GenericBound :: Trait (trait_ref)] = & opaque . bounds && let Some (segment) = trait_ref . trait_ref . path . segments . last () && let Some (generic_args) = segment . args && let [constraint] = generic_args . constraints && let Some (ty) = constraint . ty () && let TyKind :: Path (QPath :: Resolved (_ , path)) = ty . kind && let [segment] = path . segments { return Some (segment) ; } None }
};
}
