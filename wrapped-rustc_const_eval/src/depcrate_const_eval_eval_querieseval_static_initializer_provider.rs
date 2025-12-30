// Generated macro for eval_static_initializer_provider (function)
macro_rules! Depcrate_const_eval_eval_querieseval_static_initializer_provider {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"eval_static_initializer_provider"}
// Dependencies: {}
# [instrument (skip (tcx) , level = "debug")] pub fn eval_static_initializer_provider < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> :: rustc_middle :: mir :: interpret :: EvalStaticInitializerRawResult < 'tcx > { assert ! (tcx . is_static (def_id . to_def_id ())) ; let instance = ty :: Instance :: mono (tcx , def_id . to_def_id ()) ; let cid = rustc_middle :: mir :: interpret :: GlobalId { instance , promoted : None } ; eval_in_interpreter (tcx , cid , ty :: TypingEnv :: fully_monomorphized ()) }
};
}
