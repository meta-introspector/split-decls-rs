// Generated macro for LogStream (struct)
macro_rules! Depcrate_stream_logLogStream {
() => {
// Module: crate::stream::log
// Provides: {"LogStream"}
// Dependencies: {}
# [doc = " LogStream a IO stream wrapper,"] # [doc = " which logs each write/read operation."] # [derive (Debug)] pub struct LogStream < S , W > { stream : S , logger : W , }
};
}
