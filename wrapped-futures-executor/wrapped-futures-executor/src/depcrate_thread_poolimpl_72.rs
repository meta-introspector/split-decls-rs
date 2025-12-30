// Generated macro for impl_72 (impl)
macro_rules! Depcrate_thread_poolimpl_72 {
() => {
// Module: crate::thread_pool
// Provides: {"impl_72"}
// Dependencies: {}
impl Task { # [doc = " Actually run the task (invoking `poll` on the future) on the current"] # [doc = " thread."] fn run (self) { let Self { mut future , wake_handle , mut exec } = self ; let waker = waker_ref (& wake_handle) ; let mut cx = Context :: from_waker (& waker) ; unsafe { wake_handle . mutex . start_poll () ; loop { let res = future . poll_unpin (& mut cx) ; match res { Poll :: Pending => { } Poll :: Ready (()) => return wake_handle . mutex . complete () , } let task = Self { future , wake_handle : wake_handle . clone () , exec } ; match wake_handle . mutex . wait (task) { Ok (()) => return , Err (task) => { future = task . future ; exec = task . exec ; } } } } } }
};
}
