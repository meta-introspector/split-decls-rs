// Generated macro for impl_1299 (impl)
macro_rules! Depcrate_stream_try_stream_try_flattenimpl_1299 {
() => {
// Module: crate::stream::try_stream::try_flatten
// Provides: {"impl_1299"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for TryFlatten < S > where S : TryStream + Sink < Item > , { type Error = < S as Sink < Item > > :: Error ; delegate_sink ! (stream , Item) ; }
};
}
