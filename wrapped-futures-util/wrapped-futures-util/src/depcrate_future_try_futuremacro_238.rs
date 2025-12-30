// Generated macro for macro_238 (macro)
macro_rules! Depcrate_future_try_futuremacro_238 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_238"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`try_flatten_err`](TryFutureExt::try_flatten_err) method."] TryFlattenErr < Fut1 , Fut2 > (try_flatten_err :: TryFlattenErr < Fut1 , Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 | try_flatten_err :: TryFlattenErr :: new (x)]) ;
};
}
