// Generated macro for impl_603 (impl)
macro_rules! Depcrate_stream_stream_cycleimpl_603 {
() => {
// Module: crate::stream::stream::cycle
// Provides: {"impl_603"}
// Dependencies: {}
impl < St > Cycle < St > where St : Clone + Stream , { pub (super) fn new (stream : St) -> Self { Self { orig : stream . clone () , stream } } }
};
}
