// Generated macro for impl_604 (impl)
macro_rules! Depcrate_stream_stream_cycleimpl_604 {
() => {
// Module: crate::stream::stream::cycle
// Provides: {"impl_604"}
// Dependencies: {}
impl < St > Stream for Cycle < St > where St : Clone + Stream , { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; match ready ! (this . stream . as_mut () . poll_next (cx)) { None => { this . stream . set (this . orig . clone ()) ; this . stream . poll_next (cx) } item => Poll :: Ready (item) , } } fn size_hint (& self) -> (usize , Option < usize >) { match self . orig . size_hint () { size @ (0 , Some (0)) => size , (0 , _) => (0 , None) , _ => (usize :: MAX , None) , } } }
};
}
