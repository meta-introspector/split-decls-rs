// Generated macro for macro_250 (macro)
macro_rules! Depcrate_future_try_futuremacro_250 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_250"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`map_ok_or_else`](TryFutureExt::map_ok_or_else) method."] MapOkOrElse < Fut , F , G > (Map < IntoFuture < Fut >, MapOkOrElseFn < F , G >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F , g : G | Map :: new (IntoFuture :: new (x) , map_ok_or_else_fn (f , g))]) ;
};
}
