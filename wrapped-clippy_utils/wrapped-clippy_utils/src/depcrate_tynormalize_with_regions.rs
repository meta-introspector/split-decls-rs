// Generated macro for normalize_with_regions (function)
macro_rules! Depcrate_tynormalize_with_regions {
() => {
// Module: crate::ty
// Provides: {"normalize_with_regions"}
// Dependencies: {}
pub fn normalize_with_regions < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx >) -> Ty < 'tcx > { let cause = ObligationCause :: dummy () ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; infcx . at (& cause , param_env) . query_normalize (ty) . map_or (ty , | ty | ty . value) }
};
}
