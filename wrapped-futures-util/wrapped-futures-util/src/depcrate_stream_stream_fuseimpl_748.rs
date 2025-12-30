// Generated macro for impl_748 (impl)
macro_rules! Depcrate_stream_stream_fuseimpl_748 {
() => {
// Module: crate::stream::stream::fuse
// Provides: {"impl_748"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S : Stream + Sink < Item > , Item > Sink < Item > for Fuse < S > { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
