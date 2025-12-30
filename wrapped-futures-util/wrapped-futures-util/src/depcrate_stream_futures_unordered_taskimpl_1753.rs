// Generated macro for impl_1753 (impl)
macro_rules! Depcrate_stream_futures_unordered_taskimpl_1753 {
() => {
// Module: crate::stream::futures_unordered::task
// Provides: {"impl_1753"}
// Dependencies: {}
impl < Fut > Drop for Task < Fut > { fn drop (& mut self) { unsafe { if (* self . future . get ()) . is_some () { abort ("future still here when dropping") ; } } } }
};
}
