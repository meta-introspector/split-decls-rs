// Generated macro for impl_11 (impl)
macro_rules! Depcrate_async_streamimpl_11 {
() => {
// Module: crate::async_stream
// Provides: {"impl_11"}
// Dependencies: {}
impl < T , U > Stream for AsyncStream < T , U > where U : Future < Output = () > , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let me = self . project () ; if * me . done { return Poll :: Ready (None) ; } let mut dst = None ; let res = { let _enter = me . rx . enter (& mut dst) ; me . generator . poll (cx) } ; * me . done = res . is_ready () ; if dst . is_some () { return Poll :: Ready (dst . take ()) ; } if * me . done { Poll :: Ready (None) } else { Poll :: Pending } } fn size_hint (& self) -> (usize , Option < usize >) { if self . done { (0 , Some (0)) } else { (0 , None) } } }
};
}
