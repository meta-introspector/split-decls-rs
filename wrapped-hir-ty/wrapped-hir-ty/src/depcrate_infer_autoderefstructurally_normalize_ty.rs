// Generated macro for structurally_normalize_ty (function)
macro_rules! Depcrate_infer_autoderefstructurally_normalize_ty {
() => {
// Module: crate::infer::autoderef
// Provides: {"structurally_normalize_ty"}
// Dependencies: {}
fn structurally_normalize_ty < 'db > (infcx : & InferCtxt < 'db > , param_env : ParamEnv < 'db > , ty : Ty < 'db > ,) -> Option < (Ty < 'db > , PredicateObligations < 'db >) > { let mut ocx = ObligationCtxt :: new (infcx) ; let Ok (normalized_ty) = ocx . structurally_normalize_ty (& ObligationCause :: misc () , param_env , ty) else { return None ; } ; let errors = ocx . try_evaluate_obligations () ; if ! errors . is_empty () { unreachable ! () ; } Some ((normalized_ty , ocx . into_pending_obligations ())) }
};
}
