// Generated macro for adt_def_id (function)
macro_rules! Depcrate_ty_type_certaintyadt_def_id {
() => {
// Module: crate::ty::type_certainty
// Provides: {"adt_def_id"}
// Dependencies: {}
fn adt_def_id (ty : Ty < '_ >) -> Option < DefId > { ty . peel_refs () . ty_adt_def () . map (AdtDef :: did) }
};
}
