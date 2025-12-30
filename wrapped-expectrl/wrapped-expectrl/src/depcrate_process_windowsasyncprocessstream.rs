// Generated macro for AsyncProcessStream (struct)
macro_rules! Depcrate_process_windowsAsyncProcessStream {
() => {
// Module: crate::process::windows
// Provides: {"AsyncProcessStream"}
// Dependencies: {}
# [doc = " An async version of IO stream of [WinProcess]."] # [cfg (feature = "async")] # [derive (Debug)] pub struct AsyncProcessStream { output : blocking :: Unblock < PipeReader > , input : blocking :: Unblock < PipeWriter > , }
};
}
