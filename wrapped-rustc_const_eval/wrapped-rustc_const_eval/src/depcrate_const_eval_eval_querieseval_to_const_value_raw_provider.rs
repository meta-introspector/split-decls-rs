// Generated macro for eval_to_const_value_raw_provider (function)
macro_rules! Depcrate_const_eval_eval_querieseval_to_const_value_raw_provider {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"eval_to_const_value_raw_provider"}
// Dependencies: {}
# [instrument (skip (tcx) , level = "debug")] pub fn eval_to_const_value_raw_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , GlobalId < 'tcx > > ,) -> :: rustc_middle :: mir :: interpret :: EvalToConstValueResult < 'tcx > { tcx . eval_to_allocation_raw (key) . map (| val | turn_into_const_value (tcx , val , key)) }
};
}
