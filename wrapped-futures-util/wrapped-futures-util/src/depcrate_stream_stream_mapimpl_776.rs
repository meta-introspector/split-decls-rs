// Generated macro for impl_776 (impl)
macro_rules! Depcrate_stream_stream_mapimpl_776 {
() => {
// Module: crate::stream::stream::map
// Provides: {"impl_776"}
// Dependencies: {}
impl < St , F > Stream for Map < St , F > where St : Stream , F : FnMut1 < St :: Item > , { type Item = F :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let res = ready ! (this . stream . as_mut () . poll_next (cx)) ; Poll :: Ready (res . map (| x | this . f . call_mut (x))) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
};
}
