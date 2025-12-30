// Generated macro for impl_665 (impl)
macro_rules! Depcrate_stream_stream_flattenimpl_665 {
() => {
// Module: crate::stream::stream::flatten
// Provides: {"impl_665"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Flatten < S , S :: Item > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
