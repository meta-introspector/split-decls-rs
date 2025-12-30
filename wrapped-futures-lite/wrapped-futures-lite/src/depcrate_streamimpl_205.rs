// Generated macro for impl_205 (impl)
macro_rules! Depcrate_streamimpl_205 {
() => {
// Module: crate::stream
// Provides: {"impl_205"}
// Dependencies: {}
impl < S , F , E > Future for TryForEachFuture < '_ , S , F > where S : Stream + Unpin + ? Sized , F : FnMut (S :: Item) -> Result < () , E > , { type Output = Result < () , E > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { None => return Poll :: Ready (Ok (())) , Some (v) => (& mut self . f) (v) ? , } } } }
};
}
