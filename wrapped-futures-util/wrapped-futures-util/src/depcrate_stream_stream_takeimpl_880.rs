// Generated macro for impl_880 (impl)
macro_rules! Depcrate_stream_stream_takeimpl_880 {
() => {
// Module: crate::stream::stream::take
// Provides: {"impl_880"}
// Dependencies: {}
impl < St > FusedStream for Take < St > where St : FusedStream , { fn is_terminated (& self) -> bool { self . remaining == 0 || self . stream . is_terminated () } }
};
}
