// Generated macro for impl_312 (impl)
macro_rules! Depcrate_concurrent_stream_limitimpl_312 {
() => {
// Module: crate::concurrent_stream::limit
// Provides: {"impl_312"}
// Dependencies: {}
impl < CS : ConcurrentStream > Limit < CS > { pub (crate) fn new (inner : CS , limit : Option < NonZeroUsize >) -> Self { Self { inner , limit } } }
};
}
