// Generated macro for impl_1284 (impl)
macro_rules! Depcrate_stream_try_stream_try_filter_mapimpl_1284 {
() => {
// Module: crate::stream::try_stream::try_filter_map
// Provides: {"impl_1284"}
// Dependencies: {}
impl < St , Fut , F , T > FusedStream for TryFilterMap < St , Fut , F > where St : TryStream + FusedStream , Fut : TryFuture < Ok = Option < T > , Error = St :: Error > , F : FnMut (St :: Ok) -> Fut , { fn is_terminated (& self) -> bool { self . pending . is_none () && self . stream . is_terminated () } }
};
}
