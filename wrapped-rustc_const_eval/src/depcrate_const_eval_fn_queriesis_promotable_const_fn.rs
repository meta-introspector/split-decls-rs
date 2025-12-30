// Generated macro for is_promotable_const_fn (function)
macro_rules! Depcrate_const_eval_fn_queriesis_promotable_const_fn {
() => {
// Module: crate::const_eval::fn_queries
// Provides: {"is_promotable_const_fn"}
// Dependencies: {}
fn is_promotable_const_fn (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . is_const_fn (def_id) && match tcx . lookup_const_stability (def_id) { Some (stab) => { if cfg ! (debug_assertions) && stab . promotable { let sig = tcx . fn_sig (def_id) ; assert ! (sig . skip_binder () . safety () . is_safe () , "don't mark const unsafe fns as promotable" ,) ; } stab . promotable } None => false , } }
};
}
