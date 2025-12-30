// Generated macro for impl_141 (impl)
macro_rules! Depcrate_produce_iceimpl_141 {
() => {
// Module: crate::produce_ice
// Provides: {"impl_141"}
// Dependencies: {}
impl EarlyLintPass for ProduceIce { fn check_fn (& mut self , ctx : & EarlyContext < '_ > , fn_kind : FnKind < '_ > , span : Span , _ : NodeId) { if is_trigger_fn (fn_kind) { ctx . sess () . dcx () . span_delayed_bug (span , "Would you like some help with that?") ; } } }
};
}
