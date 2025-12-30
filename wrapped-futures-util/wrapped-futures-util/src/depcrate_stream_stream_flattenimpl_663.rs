// Generated macro for impl_663 (impl)
macro_rules! Depcrate_stream_stream_flattenimpl_663 {
() => {
// Module: crate::stream::stream::flatten
// Provides: {"impl_663"}
// Dependencies: {}
impl < St > FusedStream for Flatten < St , St :: Item > where St : FusedStream , St :: Item : Stream , { fn is_terminated (& self) -> bool { self . next . is_none () && self . stream . is_terminated () } }
};
}
