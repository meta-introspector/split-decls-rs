// Generated macro for never_type_behavior (function)
macro_rules! Depcrate_fn_ctxtnever_type_behavior {
() => {
// Module: crate::fn_ctxt
// Provides: {"never_type_behavior"}
// Dependencies: {}
fn never_type_behavior (tcx : TyCtxt < '_ >) -> (DivergingFallbackBehavior , DivergingBlockBehavior) { let (fallback , block) = parse_never_type_options_attr (tcx) ; let fallback = fallback . unwrap_or_else (| | default_fallback (tcx)) ; let block = block . unwrap_or_default () ; (fallback , block) }
};
}
