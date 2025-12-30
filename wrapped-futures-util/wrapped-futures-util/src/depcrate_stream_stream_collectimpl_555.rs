// Generated macro for impl_555 (impl)
macro_rules! Depcrate_stream_stream_collectimpl_555 {
() => {
// Module: crate::stream::stream::collect
// Provides: {"impl_555"}
// Dependencies: {}
impl < St , C > Future for Collect < St , C > where St : Stream , C : Default + Extend < St :: Item > , { type Output = C ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < C > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (e) => this . collection . extend (Some (e)) , None => return Poll :: Ready (self . finish ()) , } } } }
};
}
