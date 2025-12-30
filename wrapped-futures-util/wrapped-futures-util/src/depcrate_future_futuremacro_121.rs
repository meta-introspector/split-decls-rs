// Generated macro for macro_121 (macro)
macro_rules! Depcrate_future_futuremacro_121 {
() => {
// Module: crate::future::future
// Provides: {"macro_121"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`never_error`](super::FutureExt::never_error) combinator."] NeverError < Fut > (Map < Fut , OkFn < Infallible >>) : Debug + Future + FusedFuture + New [| x : Fut | Map :: new (x , ok_fn ())]) ;
};
}
