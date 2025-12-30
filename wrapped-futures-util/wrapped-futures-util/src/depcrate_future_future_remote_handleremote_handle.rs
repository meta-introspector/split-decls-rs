// Generated macro for remote_handle (function)
macro_rules! Depcrate_future_future_remote_handleremote_handle {
() => {
// Module: crate::future::future::remote_handle
// Provides: {"remote_handle"}
// Dependencies: {}
pub (super) fn remote_handle < Fut : Future > (future : Fut) -> (Remote < Fut > , RemoteHandle < Fut :: Output >) { let (tx , rx) = oneshot :: channel () ; let keep_running = Arc :: new (AtomicBool :: new (false)) ; let wrapped = Remote { future : AssertUnwindSafe (future) . catch_unwind () , tx : Some (tx) , keep_running : keep_running . clone () , } ; (wrapped , RemoteHandle { rx , keep_running }) }
};
}
