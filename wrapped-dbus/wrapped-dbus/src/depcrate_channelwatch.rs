// Generated macro for Watch (struct)
macro_rules! Depcrate_channelWatch {
() => {
// Module: crate::channel
// Provides: {"Watch"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] # [doc = " A file descriptor, and an indication whether it should be read from, written to, or both."] pub struct Watch { # [doc = " File descriptor"] pub fd : WatchFd , # [doc = " True if wakeup should happen when the file descriptor is ready for reading"] pub read : bool , # [doc = " True if wakeup should happen when the file descriptor is ready for writing"] pub write : bool , }
};
}
