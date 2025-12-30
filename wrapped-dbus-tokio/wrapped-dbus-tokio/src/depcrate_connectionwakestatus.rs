// Generated macro for WakeStatus (enum)
macro_rules! Depcrate_connectionWakeStatus {
() => {
// Module: crate::connection
// Provides: {"WakeStatus"}
// Dependencies: {}
# [derive (Debug)] enum WakeStatus { # [doc = " The resource task has not yet been polled; ready is false if it has never been polled"] # [doc = " before, otherwise, it's being woken as a result of the Channel calling the waker callback"] Waiting { ready : bool } , # [doc = " The resource task is ready to be woken again"] Polled { waker : task :: Waker } , }
};
}
