// Generated macro for is_structural_partial_eq (function)
macro_rules! Depcrate_equatable_if_letis_structural_partial_eq {
() => {
// Module: crate::equatable_if_let
// Provides: {"is_structural_partial_eq"}
// Dependencies: {}
fn is_structural_partial_eq < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , other : Ty < 'tcx >) -> bool { if let Some (def_id) = cx . tcx . lang_items () . eq_trait () { implements_trait (cx , ty , def_id , & [other . into ()]) } else { false } }
};
}
