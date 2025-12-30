// Generated macro for impl_1121 (impl)
macro_rules! Depcrate_stream_stream_splitimpl_1121 {
() => {
// Module: crate::stream::stream::split
// Provides: {"impl_1121"}
// Dependencies: {}
impl < S : Stream > Stream for SplitStream < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { ready ! (self . 0 . poll_lock (cx)) . as_pin_mut () . poll_next (cx) } }
};
}
