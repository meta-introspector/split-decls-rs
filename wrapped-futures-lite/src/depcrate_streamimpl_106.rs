// Generated macro for impl_106 (impl)
macro_rules! Depcrate_streamimpl_106 {
() => {
// Module: crate::stream
// Provides: {"impl_106"}
// Dependencies: {}
impl < F : Future > Stream for OnceFuture < F > { type Item = F :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; match this . future . as_mut () . as_pin_mut () . map (| f | f . poll (cx)) { Some (Poll :: Ready (t)) => { this . future . set (None) ; Poll :: Ready (Some (t)) } Some (Poll :: Pending) => Poll :: Pending , None => Poll :: Ready (None) , } } }
};
}
