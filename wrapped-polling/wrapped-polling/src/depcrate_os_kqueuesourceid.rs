// Generated macro for SourceId (enum)
macro_rules! Depcrate_os_kqueueSourceId {
() => {
// Module: crate::os::kqueue
// Provides: {"SourceId"}
// Dependencies: {}
# [doc = " Identifier for a source."] # [doc (hidden)] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum SourceId { # [doc = " Registered file descriptor."] Fd (RawFd) , # [doc = " Signal."] Signal (std :: os :: raw :: c_int) , # [doc = " Process ID."] Pid (rustix :: process :: Pid) , # [doc = " Timer ID."] Timer (usize) , }
};
}
