// Generated macro for impl_650 (impl)
macro_rules! Depcrate_stream_stream_filter_mapimpl_650 {
() => {
// Module: crate::stream::stream::filter_map
// Provides: {"impl_650"}
// Dependencies: {}
impl < St , Fut , F , T > FusedStream for FilterMap < St , Fut , F > where St : Stream + FusedStream , F : FnMut1 < St :: Item , Output = Fut > , Fut : Future < Output = Option < T > > , { fn is_terminated (& self) -> bool { self . pending . is_none () && self . stream . is_terminated () } }
};
}
