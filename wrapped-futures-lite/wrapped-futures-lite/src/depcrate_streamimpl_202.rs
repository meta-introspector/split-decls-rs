// Generated macro for impl_202 (impl)
macro_rules! Depcrate_streamimpl_202 {
() => {
// Module: crate::stream
// Provides: {"impl_202"}
// Dependencies: {}
impl < S , F > Future for ForEachFuture < S , F > where S : Stream , F : FnMut (S :: Item) , { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (v) => (this . f) (v) , None => return Poll :: Ready (()) , } } } }
};
}
