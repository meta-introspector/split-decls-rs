// Generated macro for impl_775 (impl)
macro_rules! Depcrate_stream_stream_mapimpl_775 {
() => {
// Module: crate::stream::stream::map
// Provides: {"impl_775"}
// Dependencies: {}
impl < St , F > FusedStream for Map < St , F > where St : FusedStream , F : FnMut1 < St :: Item > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
