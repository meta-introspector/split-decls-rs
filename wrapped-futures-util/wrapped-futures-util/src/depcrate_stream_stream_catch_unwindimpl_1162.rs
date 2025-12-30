// Generated macro for impl_1162 (impl)
macro_rules! Depcrate_stream_stream_catch_unwindimpl_1162 {
() => {
// Module: crate::stream::stream::catch_unwind
// Provides: {"impl_1162"}
// Dependencies: {}
impl < St : FusedStream + UnwindSafe > FusedStream for CatchUnwind < St > { fn is_terminated (& self) -> bool { self . caught_unwind || self . stream . is_terminated () } }
};
}
