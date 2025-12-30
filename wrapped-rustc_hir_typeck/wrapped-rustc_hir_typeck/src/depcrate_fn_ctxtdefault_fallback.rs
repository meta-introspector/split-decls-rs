// Generated macro for default_fallback (function)
macro_rules! Depcrate_fn_ctxtdefault_fallback {
() => {
// Module: crate::fn_ctxt
// Provides: {"default_fallback"}
// Dependencies: {}
# [doc = " Returns the default fallback which is used when there is no explicit override via `#![never_type_options(...)]`."] fn default_fallback (tcx : TyCtxt < '_ >) -> DivergingFallbackBehavior { if tcx . sess . edition () . at_least_rust_2024 () { return DivergingFallbackBehavior :: ToNever ; } if tcx . features () . never_type_fallback () { return DivergingFallbackBehavior :: ContextDependent ; } DivergingFallbackBehavior :: ToUnit }
};
}
