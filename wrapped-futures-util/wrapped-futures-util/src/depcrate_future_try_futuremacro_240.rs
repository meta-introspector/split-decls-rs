// Generated macro for macro_240 (macro)
macro_rules! Depcrate_future_try_futuremacro_240 {
() => {
// Module: crate::future::try_future
// Provides: {"macro_240"}
// Dependencies: {}
# [cfg (feature = "sink")] delegate_all ! (# [doc = " Sink for the [`flatten_sink`](TryFutureExt::flatten_sink) method."] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] FlattenSink < Fut , Si > (try_flatten :: TryFlatten < Fut , Si >) : Debug + Sink + Stream + FusedStream + New [| x : Fut | try_flatten :: TryFlatten :: new (x)]) ;
};
}
