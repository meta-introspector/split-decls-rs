// Generated macro for impl_145 (impl)
macro_rules! Depcrate_produce_iceimpl_145 {
() => {
// Module: crate::produce_ice
// Provides: {"impl_145"}
// Dependencies: {}
impl EarlyLintPass for ProduceIce { fn check_fn (& mut self , cx : & EarlyContext < '_ > , fn_kind : FnKind < '_ > , span : Span , _ : NodeId) { if is_trigger_fn (fn_kind) { cx . sess () . dcx () . span_delayed_bug (span , "Would you like some help with that?") ; } } }
};
}
