// Generated macro for BacktraceFrameFmt (struct)
macro_rules! Depcrate_printBacktraceFrameFmt {
() => {
// Module: crate::print
// Provides: {"BacktraceFrameFmt"}
// Dependencies: {}
# [doc = " A formatter for just one frame of a backtrace."] # [doc = ""] # [doc = " This type is created by the `BacktraceFmt::frame` function."] pub struct BacktraceFrameFmt < 'fmt , 'a , 'b > { fmt : & 'fmt mut BacktraceFmt < 'a , 'b > , symbol_index : usize , }
};
}
