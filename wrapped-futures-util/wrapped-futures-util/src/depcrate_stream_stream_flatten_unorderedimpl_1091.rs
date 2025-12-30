// Generated macro for impl_1091 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1091 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1091"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < St , Item , Fc > Sink < Item > for FlattenUnorderedWithFlowController < St , Fc > where St : Stream + Sink < Item > , { type Error = St :: Error ; delegate_sink ! (stream , Item) ; }
};
}
