// Generated macro for impl_422 (impl)
macro_rules! Depcrate_fut_streamimpl_422 {
() => {
// Module: crate::fut::stream
// Provides: {"impl_422"}
// Dependencies: {}
impl < S , A > ActorStream < A > for StreamWrap < S , A > where S : Stream , A : Actor , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , _ : & mut A , _ : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Option < Self :: Item > > { self . project () . stream . poll_next (task) } }
};
}
