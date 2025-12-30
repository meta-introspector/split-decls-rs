// Generated macro for macro_246 (macro)
macro_rules! Depcrate_future_try_futuremacro_246 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_246"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`inspect_err`](super::TryFutureExt::inspect_err) method."] InspectErr < Fut , F > (Inspect < IntoFuture < Fut >, InspectErrFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Inspect :: new (IntoFuture :: new (x) , inspect_err_fn (f))]) ;
};
}
