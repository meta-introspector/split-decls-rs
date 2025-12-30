// Generated macro for impl_618 (impl)
macro_rules! Depcrate_stream_stream_enumerateimpl_618 {
() => {
// Module: crate::stream::stream::enumerate
// Provides: {"impl_618"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Enumerate < S > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
