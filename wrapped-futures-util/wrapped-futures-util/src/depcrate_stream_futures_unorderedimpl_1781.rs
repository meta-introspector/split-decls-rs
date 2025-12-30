// Generated macro for impl_1781 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1781 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1781"}
// Dependencies: {}
impl < Fut > Drop for FuturesUnordered < Fut > { fn drop (& mut self) { struct LeakQueueOnDrop < 'a , Fut > (& 'a mut FuturesUnordered < Fut >) ; impl < Fut > Drop for LeakQueueOnDrop < '_ , Fut > { fn drop (& mut self) { mem :: forget (Arc :: clone (& self . 0 . ready_to_run_queue)) ; } } let guard = LeakQueueOnDrop (self) ; while ! guard . 0 . head_all . get_mut () . is_null () { let head = * guard . 0 . head_all . get_mut () ; let task = unsafe { guard . 0 . unlink (head) } ; guard . 0 . release_task (task) ; } mem :: forget (guard) ; } }
};
}
