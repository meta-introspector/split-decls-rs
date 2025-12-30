// Generated macro for macro_244 (macro)
macro_rules! Depcrate_future_try_futuremacro_244 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_244"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`ok_into`](TryFutureExt::ok_into) method."] OkInto < Fut , E > (MapOk < Fut , IntoFn < E >>) : Debug + Future + FusedFuture + New [| x : Fut | MapOk :: new (x , into_fn ())]) ;
};
}
