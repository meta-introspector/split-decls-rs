// Generated macro for is_diag_trait_item (function)
macro_rules! Depcrateis_diag_trait_item {
() => {
// Module: crate
// Provides: {"is_diag_trait_item"}
// Dependencies: {}
# [doc = " Checks if a method is in a diagnostic item trait"] pub fn is_diag_trait_item (cx : & LateContext < '_ > , def_id : DefId , diag_item : Symbol) -> bool { if let Some (trait_did) = cx . tcx . trait_of_assoc (def_id) { return cx . tcx . is_diagnostic_item (diag_item , trait_did) ; } false }
};
}
