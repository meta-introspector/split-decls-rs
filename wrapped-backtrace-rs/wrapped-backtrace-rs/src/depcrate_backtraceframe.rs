// Generated macro for Frame (struct)
macro_rules! Depcrate_backtraceFrame {
() => {
// Module: crate::backtrace
// Provides: {"Frame"}
// Dependencies: {}
# [doc = " A trait representing one frame of a backtrace, yielded to the `trace`"] # [doc = " function of this crate."] # [doc = ""] # [doc = " The tracing function's closure will be yielded frames, and the frame is"] # [doc = " virtually dispatched as the underlying implementation is not always known"] # [doc = " until runtime."] # [derive (Clone)] pub struct Frame { pub (crate) inner : FrameImp , }
};
}
