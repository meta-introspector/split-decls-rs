// Generated macro for macro_237 (macro)
macro_rules! Depcrate_future_try_futuremacro_237 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_237"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`try_flatten`](TryFutureExt::try_flatten) method."] TryFlatten < Fut1 , Fut2 > (try_flatten :: TryFlatten < Fut1 , Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 | try_flatten :: TryFlatten :: new (x)]) ;
};
}
