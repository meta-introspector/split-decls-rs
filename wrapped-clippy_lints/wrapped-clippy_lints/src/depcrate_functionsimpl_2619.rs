// Generated macro for impl_2619 (impl)
macro_rules! Depcrate_functionsimpl_2619 {
() => {
// Module: crate::functions
// Provides: {"impl_2619"}
// Dependencies: {}
impl EarlyLintPass for EarlyFunctions { fn check_fn (& mut self , cx : & EarlyContext < '_ > , fn_kind : visit :: FnKind < '_ > , _ : Span , _ : ast :: NodeId) { duplicate_underscore_argument :: check (cx , fn_kind) ; } }
};
}
