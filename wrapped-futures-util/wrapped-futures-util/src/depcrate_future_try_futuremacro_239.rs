// Generated macro for macro_239 (macro)
macro_rules! Depcrate_future_try_futuremacro_239 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_239"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`try_flatten_stream`](TryFutureExt::try_flatten_stream) method."] TryFlattenStream < Fut > (try_flatten :: TryFlatten < Fut , Fut :: Ok >) : Debug + Sink + Stream + FusedStream + New [| x : Fut | try_flatten :: TryFlatten :: new (x)] where Fut : TryFuture) ;
};
}
