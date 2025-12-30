// Generated macro for impl_511 (impl)
macro_rules! Depcrate_concurrency_threadimpl_511 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_511"}
// Dependencies: {}
impl < 'tcx > ThreadState < 'tcx > { fn is_enabled (& self) -> bool { matches ! (self , ThreadState :: Enabled) } fn is_terminated (& self) -> bool { matches ! (self , ThreadState :: Terminated) } fn is_blocked_on (& self , reason : BlockReason) -> bool { matches ! (* self , ThreadState :: Blocked { reason : actual_reason , .. } if actual_reason == reason) } }
};
}
