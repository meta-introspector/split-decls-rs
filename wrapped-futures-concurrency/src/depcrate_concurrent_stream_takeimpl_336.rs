// Generated macro for impl_336 (impl)
macro_rules! Depcrate_concurrent_stream_takeimpl_336 {
() => {
// Module: crate::concurrent_stream::take
// Provides: {"impl_336"}
// Dependencies: {}
impl < CS : ConcurrentStream > Take < CS > { pub (crate) fn new (inner : CS , limit : usize) -> Self { Self { inner , limit } } }
};
}
