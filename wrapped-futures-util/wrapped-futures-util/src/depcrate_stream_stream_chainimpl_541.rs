// Generated macro for impl_541 (impl)
macro_rules! Depcrate_stream_stream_chainimpl_541 {
() => {
// Module: crate::stream::stream::chain
// Provides: {"impl_541"}
// Dependencies: {}
impl < St1 , St2 > FusedStream for Chain < St1 , St2 > where St1 : Stream , St2 : FusedStream < Item = St1 :: Item > , { fn is_terminated (& self) -> bool { self . first . is_none () && self . second . is_terminated () } }
};
}
