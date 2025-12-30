// Generated macro for join_recover_from_panic (function)
macro_rules! Depcrate_joinjoin_recover_from_panic {
() => {
// Module: crate::join
// Provides: {"join_recover_from_panic"}
// Dependencies: {}
# [doc = " If job A panics, we still cannot return until we are sure that job"] # [doc = " B is complete. This is because it may contain references into the"] # [doc = " enclosing stack frame(s)."] # [cold] unsafe fn join_recover_from_panic (worker_thread : & WorkerThread , job_b_latch : & SpinLatch < '_ > , err : Box < dyn Any + Send > ,) -> ! { unsafe { worker_thread . wait_until (job_b_latch) ; unwind :: resume_unwinding (err) } }
};
}
