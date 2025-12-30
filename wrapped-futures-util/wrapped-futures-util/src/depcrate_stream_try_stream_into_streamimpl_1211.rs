// Generated macro for impl_1211 (impl)
macro_rules! Depcrate_stream_try_stream_into_streamimpl_1211 {
() => {
// Module: crate::stream::try_stream::into_stream
// Provides: {"impl_1211"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S : Sink < Item > , Item > Sink < Item > for IntoStream < S > { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
