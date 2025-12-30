// Generated macro for macro_248 (macro)
macro_rules! Depcrate_future_try_futuremacro_248 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_248"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`map_ok`](TryFutureExt::map_ok) method."] MapOk < Fut , F > (Map < IntoFuture < Fut >, MapOkFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Map :: new (IntoFuture :: new (x) , map_ok_fn (f))]) ;
};
}
