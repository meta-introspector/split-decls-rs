// Generated macro for impl_881 (impl)
macro_rules! Depcrate_stream_wait_untilimpl_881 {
() => {
// Module: crate::stream::wait_until
// Provides: {"impl_881"}
// Dependencies: {}
impl < S , D > Stream for WaitUntil < S , D > where S : Stream , D : Future , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match this . state { State :: Timer => match this . deadline . poll (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (_) => { * this . state = State :: Streaming ; this . stream . poll_next (cx) } } , State :: Streaming => this . stream . poll_next (cx) , } } }
};
}
