// Generated macro for impl_1160 (impl)
macro_rules! Depcrate_stream_stream_catch_unwindimpl_1160 {
() => {
// Module: crate::stream::stream::catch_unwind
// Provides: {"impl_1160"}
// Dependencies: {}
impl < St : Stream + UnwindSafe > CatchUnwind < St > { pub (super) fn new (stream : St) -> Self { Self { stream , caught_unwind : false } } delegate_access_inner ! (stream , St , ()) ; }
};
}
