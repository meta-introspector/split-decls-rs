// Generated macro for impl_1210 (impl)
macro_rules! Depcrate_stream_try_stream_into_streamimpl_1210 {
() => {
// Module: crate::stream::try_stream::into_stream
// Provides: {"impl_1210"}
// Dependencies: {}
impl < St : TryStream > Stream for IntoStream < St > { type Item = Result < St :: Ok , St :: Error > ; # [inline] fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . project () . stream . try_poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
};
}
