// Generated macro for macro_249 (macro)
macro_rules! Depcrate_future_try_futuremacro_249 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_249"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`map_err`](TryFutureExt::map_err) method."] MapErr < Fut , F > (Map < IntoFuture < Fut >, MapErrFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Map :: new (IntoFuture :: new (x) , map_err_fn (f))]) ;
};
}
