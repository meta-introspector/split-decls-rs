// Generated macro for impl_596 (impl)
macro_rules! Depcrate_fn_ctxtimpl_596 {
() => {
// Module: crate::fn_ctxt
// Provides: {"impl_596"}
// Dependencies: {}
impl < 'tcx > LoweredTy < 'tcx > { fn from_raw (fcx : & FnCtxt < '_ , 'tcx > , span : Span , raw : Ty < 'tcx >) -> LoweredTy < 'tcx > { let normalized = if fcx . next_trait_solver () { fcx . try_structurally_resolve_type (span , raw) } else { fcx . normalize (span , raw) } ; LoweredTy { raw , normalized } } }
};
}
