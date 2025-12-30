// Generated macro for impl_94 (impl)
macro_rules! Depcrate_runnableimpl_94 {
() => {
// Module: crate::runnable
// Provides: {"impl_94"}
// Dependencies: {}
impl < M > Drop for Runnable < M > { fn drop (& mut self) { let ptr = self . ptr . as_ptr () ; let header = self . header () ; unsafe { let mut state = header . state . load (Ordering :: Acquire) ; loop { if state & (COMPLETED | CLOSED) != 0 { break ; } match header . state . compare_exchange_weak (state , state | CLOSED , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => break , Err (s) => state = s , } } (header . vtable . drop_future) (ptr , header . vtable . layout_info) ; let state = header . state . fetch_and (! SCHEDULED , Ordering :: AcqRel) ; if state & AWAITER != 0 { (* header) . notify (None) ; } drop_ref (ptr) ; } } }
};
}
