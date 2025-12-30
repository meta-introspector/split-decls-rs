// Generated macro for impl_820 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_820 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_820"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Peekable < S > where S : Sink < Item > + Stream , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
