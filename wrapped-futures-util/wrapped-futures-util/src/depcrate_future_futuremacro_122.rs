// Generated macro for macro_122 (macro)
macro_rules! Depcrate_future_futuremacro_122 {
() => {
// Module: crate::future::future
// Provides: {"macro_122"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`unit_error`](super::FutureExt::unit_error) combinator."] UnitError < Fut > (Map < Fut , OkFn < () >>) : Debug + Future + FusedFuture + New [| x : Fut | Map :: new (x , ok_fn ())]) ;
};
}
