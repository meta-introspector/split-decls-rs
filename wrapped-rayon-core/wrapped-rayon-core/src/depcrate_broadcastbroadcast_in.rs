// Generated macro for broadcast_in (function)
macro_rules! Depcrate_broadcastbroadcast_in {
() => {
// Module: crate::broadcast
// Provides: {"broadcast_in"}
// Dependencies: {}
# [doc = " Execute `op` on every thread in the pool. It will be executed on each"] # [doc = " thread when they have nothing else to do locally, before they try to"] # [doc = " steal work from other threads. This function will not return until all"] # [doc = " threads have completed the `op`."] # [doc = ""] # [doc = " Unsafe because `registry` must not yet have terminated."] pub (super) unsafe fn broadcast_in < OP , R > (op : OP , registry : & Arc < Registry >) -> Vec < R > where OP : Fn (BroadcastContext < '_ >) -> R + Sync , R : Send , { unsafe { let f = move | injected : bool | { debug_assert ! (injected) ; BroadcastContext :: with (& op) } ; let n_threads = registry . num_threads () ; let current_thread = WorkerThread :: current () . as_ref () ; let latch = CountLatch :: with_count (n_threads , current_thread) ; let jobs : Vec < _ > = (0 .. n_threads) . map (| _ | StackJob :: new (& f , LatchRef :: new (& latch))) . collect () ; let job_refs = jobs . iter () . map (| job | job . as_job_ref ()) ; registry . inject_broadcast (job_refs) ; latch . wait (current_thread) ; jobs . into_iter () . map (| job | job . into_result ()) . collect () } }
};
}
