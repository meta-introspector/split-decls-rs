// Generated macro for macro_245 (macro)
macro_rules! Depcrate_future_try_futuremacro_245 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_245"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`inspect_ok`](super::TryFutureExt::inspect_ok) method."] InspectOk < Fut , F > (Inspect < IntoFuture < Fut >, InspectOkFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | Inspect :: new (IntoFuture :: new (x) , inspect_ok_fn (f))]) ;
};
}
