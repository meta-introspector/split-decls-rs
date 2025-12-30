// Generated macro for EventState (struct)
macro_rules! Depcrate_fd_eventfdEventState {
() => {
// Module: crate::fd::eventfd
// Provides: {"EventState"}
// Dependencies: {}
# [derive (Debug)] struct EventState { pub counter : u64 , pub read_queue : VecDeque < Waker > , pub write_queue : VecDeque < Waker > , }
};
}
