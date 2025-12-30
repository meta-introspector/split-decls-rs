// Generated macro for anonymize_predicate (function)
macro_rules! Depcrate_traits_utilanonymize_predicate {
() => {
// Module: crate::traits::util
// Provides: {"anonymize_predicate"}
// Dependencies: {}
pub fn anonymize_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , pred : ty :: Predicate < 'tcx > ,) -> ty :: Predicate < 'tcx > { let new = tcx . anonymize_bound_vars (pred . kind ()) ; tcx . reuse_or_mk_predicate (pred , new) }
};
}
