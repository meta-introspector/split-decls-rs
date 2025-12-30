// Generated macro for WaitState (enum)
macro_rules! Depcrate_barrierWaitState {
() => {
// Module: crate::barrier
// Provides: {"WaitState"}
// Dependencies: {}
enum WaitState { # [doc = " We are getting the original values of the state."] Initial , # [doc = " We are waiting for the listener to complete."] Waiting { local_gen : u64 } , # [doc = " Waiting to re-acquire the lock to check the state again."] Reacquiring { local_gen : u64 } , }
};
}
