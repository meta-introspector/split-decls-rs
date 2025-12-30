// Generated macro for impl_120 (impl)
macro_rules! Depcrate_streamimpl_120 {
() => {
// Module: crate::stream
// Provides: {"impl_120"}
// Dependencies: {}
impl < T , E , S > Future for TryNextFuture < '_ , S > where S : Stream < Item = Result < T , E > > + Unpin + ? Sized , { type Output = Result < Option < T > , E > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let res = ready ! (self . stream . poll_next (cx)) ; Poll :: Ready (res . transpose ()) } }
};
}
