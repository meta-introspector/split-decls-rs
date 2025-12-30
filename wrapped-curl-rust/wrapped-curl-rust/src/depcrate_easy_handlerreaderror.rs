// Generated macro for ReadError (enum)
macro_rules! Depcrate_easy_handlerReadError {
() => {
// Module: crate::easy::handler
// Provides: {"ReadError"}
// Dependencies: {}
# [doc = " Possible error codes that can be returned from the `read_function` callback."] # [non_exhaustive] # [derive (Debug)] pub enum ReadError { # [doc = " Indicates that the connection should be aborted immediately"] Abort , # [doc = " Indicates that reading should be paused until `unpause` is called."] Pause , }
};
}
