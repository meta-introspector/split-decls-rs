// Generated macro for impl_967 (impl)
macro_rules! Depcrate_stream_stream_zipimpl_967 {
() => {
// Module: crate::stream::stream::zip
// Provides: {"impl_967"}
// Dependencies: {}
impl < St1 , St2 > FusedStream for Zip < St1 , St2 > where St1 : Stream , St2 : Stream , { fn is_terminated (& self) -> bool { self . stream1 . is_terminated () && self . stream2 . is_terminated () } }
};
}
