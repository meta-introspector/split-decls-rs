// Generated macro for impl_594 (impl)
macro_rules! Depcrate_stream_stream_countimpl_594 {
() => {
// Module: crate::stream::stream::count
// Provides: {"impl_594"}
// Dependencies: {}
impl < St : Stream > Future for Count < St > { type Output = usize ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (_) => * this . count += 1 , None => break * this . count , } }) } }
};
}
