// Generated macro for macro_242 (macro)
macro_rules! Depcrate_future_try_futuremacro_242 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_242"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`or_else`](TryFutureExt::or_else) method."] OrElse < Fut1 , Fut2 , F > (TryFlattenErr < MapErr < Fut1 , F >, Fut2 >) : Debug + Future + FusedFuture + New [| x : Fut1 , f : F | TryFlattenErr :: new (MapErr :: new (x , f))]) ;
};
}
