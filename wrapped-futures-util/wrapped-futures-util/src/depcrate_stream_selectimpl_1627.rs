// Generated macro for impl_1627 (impl)
macro_rules! Depcrate_stream_selectimpl_1627 {
() => {
// Module: crate::stream::select
// Provides: {"impl_1627"}
// Dependencies: {}
impl < St1 , St2 > Stream for Select < St1 , St2 > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { type Item = St1 :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St1 :: Item > > { let this = self . project () ; this . inner . poll_next (cx) } }
};
}
