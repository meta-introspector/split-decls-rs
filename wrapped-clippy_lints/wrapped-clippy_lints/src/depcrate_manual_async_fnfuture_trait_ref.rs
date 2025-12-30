// Generated macro for future_trait_ref (function)
macro_rules! Depcrate_manual_async_fnfuture_trait_ref {
() => {
// Module: crate::manual_async_fn
// Provides: {"future_trait_ref"}
// Dependencies: {}
fn future_trait_ref < 'tcx > (cx : & LateContext < 'tcx > , opaque : & 'tcx OpaqueTy < 'tcx >) -> Option < & 'tcx TraitRef < 'tcx > > { if let Some (trait_ref) = opaque . bounds . iter () . find_map (| bound | { if let GenericBound :: Trait (poly) = bound { Some (& poly . trait_ref) } else { None } }) && trait_ref . trait_def_id () == cx . tcx . lang_items () . future_trait () { return Some (trait_ref) ; } None }
};
}
