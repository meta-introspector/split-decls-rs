// Generated macro for macro_243 (macro)
macro_rules! Depcrate_future_try_futuremacro_243 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_243"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`err_into`](TryFutureExt::err_into) method."] ErrInto < Fut , E > (MapErr < Fut , IntoFn < E >>) : Debug + Future + FusedFuture + New [| x : Fut | MapErr :: new (x , into_fn ())]) ;
};
}
