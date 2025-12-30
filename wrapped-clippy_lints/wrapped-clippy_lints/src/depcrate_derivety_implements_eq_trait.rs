// Generated macro for ty_implements_eq_trait (function)
macro_rules! Depcrate_derivety_implements_eq_trait {
() => {
// Module: crate::derive
// Provides: {"ty_implements_eq_trait"}
// Dependencies: {}
fn ty_implements_eq_trait < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , eq_trait_id : DefId) -> bool { tcx . non_blanket_impls_for_ty (eq_trait_id , ty) . next () . is_some () }
};
}
