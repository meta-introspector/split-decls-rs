// Generated macro for impl_1229 (impl)
macro_rules! Depcrate_stream_try_stream_or_elseimpl_1229 {
() => {
// Module: crate::stream::try_stream::or_else
// Provides: {"impl_1229"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for OrElse < S , Fut , F > where S : Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
};
}
