// Generated macro for AsyncPtyStream (struct)
macro_rules! Depcrate_process_unixAsyncPtyStream {
() => {
// Module: crate::process::unix
// Provides: {"AsyncPtyStream"}
// Dependencies: {}
# [doc = " An async version of IO stream of [UnixProcess]."] # [cfg (feature = "async")] # [derive (Debug)] pub struct AsyncPtyStream { stream : async_io :: Async < PtyStream > , }
};
}
