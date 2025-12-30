// Generated macro for macro_241 (macro)
macro_rules! Depcrate_future_try_futuremacro_241 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_241"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`and_then`](TryFutureExt::and_then) method."] AndThen < Fut1 , Fut2 , F > (TryFlatten < MapOk < Fut1 , F >, Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 , f : F | TryFlatten :: new (MapOk :: new (x , f))]) ;
};
}
