// Generated macro for eval_to_allocation_raw_provider (function)
macro_rules! Depcrate_const_eval_eval_querieseval_to_allocation_raw_provider {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"eval_to_allocation_raw_provider"}
// Dependencies: {}
# [instrument (skip (tcx) , level = "debug")] pub fn eval_to_allocation_raw_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , GlobalId < 'tcx > > ,) -> :: rustc_middle :: mir :: interpret :: EvalToAllocationRawResult < 'tcx > { assert ! (key . value . promoted . is_some () || ! tcx . is_static (key . value . instance . def_id ())) ; debug_assert_eq ! (key . typing_env . typing_mode , ty :: TypingMode :: PostAnalysis) ; if cfg ! (debug_assertions) { let instance = with_no_trimmed_paths ! (key . value . instance . to_string ()) ; trace ! ("const eval: {:?} ({})" , key , instance) ; } eval_in_interpreter (tcx , key . value , key . typing_env) }
};
}
