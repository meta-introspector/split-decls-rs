// Generated macro for impl_1768 (impl)
macro_rules! Depcrate_stream_futures_unordered_ready_to_run_queueimpl_1768 {
() => {
// Module: crate::stream::futures_unordered::ready_to_run_queue
// Provides: {"impl_1768"}
// Dependencies: {}
impl < Fut > Drop for ReadyToRunQueue < Fut > { fn drop (& mut self) { unsafe { loop { match self . dequeue () { Dequeue :: Empty => break , Dequeue :: Inconsistent => abort ("inconsistent in drop") , Dequeue :: Data (ptr) => drop (Arc :: from_raw (ptr)) , } } } } }
};
}
