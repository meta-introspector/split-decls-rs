// Generated macro for impl_178 (impl)
macro_rules! Depcrate_streamimpl_178 {
() => {
// Module: crate::stream
// Provides: {"impl_178"}
// Dependencies: {}
impl < S > Stream for Enumerate < S > where S : Stream , { type Item = (usize , S :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . stream . poll_next (cx)) { Some (v) => { let ret = (* this . i , v) ; * this . i += 1 ; Poll :: Ready (Some (ret)) } None => Poll :: Ready (None) , } } }
};
}
