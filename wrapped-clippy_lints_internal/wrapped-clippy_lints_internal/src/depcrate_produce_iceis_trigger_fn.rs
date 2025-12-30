// Generated macro for is_trigger_fn (function)
macro_rules! Depcrate_produce_iceis_trigger_fn {
() => {
// Module: crate::produce_ice
// Provides: {"is_trigger_fn"}
// Dependencies: {}
fn is_trigger_fn (fn_kind : FnKind < '_ >) -> bool { match fn_kind { FnKind :: Fn (_ , _ , func) => func . ident . name . as_str () == "it_looks_like_you_are_trying_to_kill_clippy" , FnKind :: Closure (..) => false , } }
};
}
