// Generated macro for impl_777 (impl)
macro_rules! Depcrate_stream_stream_mapimpl_777 {
() => {
// Module: crate::stream::stream::map
// Provides: {"impl_777"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < St , F , Item > Sink < Item > for Map < St , F > where St : Stream + Sink < Item > , F : FnMut1 < St :: Item > , { type Error = St :: Error ; delegate_sink ! (stream , Item) ; }
};
}
