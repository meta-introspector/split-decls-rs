// Generated macro for symmetric_partial_eq (function)
macro_rules! Depcrate_operators_cmp_ownedsymmetric_partial_eq {
() => {
// Module: crate::operators::cmp_owned
// Provides: {"symmetric_partial_eq"}
// Dependencies: {}
fn symmetric_partial_eq < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , other : Ty < 'tcx >) -> Option < EqImpl > { cx . tcx . lang_items () . eq_trait () . map (| def_id | EqImpl { ty_eq_other : implements_trait (cx , ty , def_id , & [other . into ()]) , other_eq_ty : implements_trait (cx , other , def_id , & [ty . into ()]) , }) }
};
}
