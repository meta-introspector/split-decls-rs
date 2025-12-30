// Generated macro for WaitableState (struct)
macro_rules! Depcrate_os_iocpWaitableState {
() => {
// Module: crate::os::iocp
// Provides: {"WaitableState"}
// Dependencies: {}
# [doc = " Per-waitable handle state."] # [derive (Debug)] struct WaitableState { # [doc = " The handle that this state is for."] handle : RawHandle , # [doc = " The IO completion port that this handle is registered with."] port : Weak < IoCompletionPort < Packet > > , # [doc = " The event that this handle will report."] interest : Event , # [doc = " The current poll mode."] mode : PollMode , # [doc = " The status of this waitable."] status : WaitableStatus , }
};
}
