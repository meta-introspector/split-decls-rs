// Generated macro for is_diag_item_method (function)
macro_rules! Depcrateis_diag_item_method {
() => {
// Module: crate
// Provides: {"is_diag_item_method"}
// Dependencies: {}
# [doc = " Checks if a method is defined in an impl of a diagnostic item"] pub fn is_diag_item_method (cx : & LateContext < '_ > , def_id : DefId , diag_item : Symbol) -> bool { if let Some (impl_did) = cx . tcx . impl_of_assoc (def_id) && let Some (adt) = cx . tcx . type_of (impl_did) . instantiate_identity () . ty_adt_def () { return cx . tcx . is_diagnostic_item (diag_item , adt . did ()) ; } false }
};
}
