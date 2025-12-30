// Generated macro for macro_251 (macro)
macro_rules! Depcrate_future_try_futuremacro_251 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_251"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`unwrap_or_else`](TryFutureExt::unwrap_or_else) method."] UnwrapOrElse < Fut , F > (Map < IntoFuture < Fut >, UnwrapOrElseFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Map :: new (IntoFuture :: new (x) , unwrap_or_else_fn (f))]) ;
};
}
