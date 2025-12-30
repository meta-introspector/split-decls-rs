// Generated macro for CompletionStatus (struct)
macro_rules! Depcrate_iocpCompletionStatus {
() => {
// Module: crate::iocp
// Provides: {"CompletionStatus"}
// Dependencies: {}
# [doc = " A status message received from an I/O completion port."] # [doc = ""] # [doc = " These statuses can be created via the `new` or `empty` constructors and then"] # [doc = " provided to a completion port, or they are read out of a completion port."] # [doc = " The fields of each status are read through its accessor methods."] # [derive (Clone , Copy)] # [repr (transparent)] pub struct CompletionStatus (OVERLAPPED_ENTRY) ;
};
}
