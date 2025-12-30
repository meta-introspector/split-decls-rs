// Generated macro for is_any_trait (function)
macro_rules! Depcrate_types_borrowed_boxis_any_trait {
() => {
// Module: crate::types::borrowed_box
// Provides: {"is_any_trait"}
// Dependencies: {}
fn is_any_trait (cx : & LateContext < '_ > , t : & hir :: Ty < '_ >) -> bool { if let TyKind :: TraitObject (traits , ..) = t . kind { return traits . iter () . any (| bound | { if let Some (trait_did) = bound . trait_ref . trait_def_id () && cx . tcx . is_diagnostic_item (sym :: Any , trait_did) { return true ; } false }) ; } false }
};
}
