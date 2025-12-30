// Generated macro for impl_1033 (impl)
macro_rules! Depcrate_stream_stream_buffer_unorderedimpl_1033 {
() => {
// Module: crate::stream::stream::buffer_unordered
// Provides: {"impl_1033"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for BufferUnordered < S > where S : Stream + Sink < Item > , S :: Item : Future , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
