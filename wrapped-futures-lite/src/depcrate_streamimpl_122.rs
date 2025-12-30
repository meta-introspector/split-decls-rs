// Generated macro for impl_122 (impl)
macro_rules! Depcrate_streamimpl_122 {
() => {
// Module: crate::stream
// Provides: {"impl_122"}
// Dependencies: {}
impl < S : Stream + ? Sized > Future for CountFuture < S > { type Output = usize ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . as_mut () . project () . stream . poll_next (cx)) { None => return Poll :: Ready (self . count) , Some (_) => * self . as_mut () . project () . count += 1 , } } } }
};
}
