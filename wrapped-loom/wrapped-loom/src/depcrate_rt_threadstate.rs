// Generated macro for State (enum)
macro_rules! Depcrate_rt_threadState {
() => {
// Module: crate::rt::thread
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] pub (crate) enum State { Runnable { unparked : bool } , Blocked (# [allow (dead_code)] Location) , Yield , Terminated , }
};
}
