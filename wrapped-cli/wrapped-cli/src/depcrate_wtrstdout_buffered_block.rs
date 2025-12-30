// Generated macro for stdout_buffered_block (function)
macro_rules! Depcrate_wtrstdout_buffered_block {
() => {
// Module: crate::wtr
// Provides: {"stdout_buffered_block"}
// Dependencies: {}
# [doc = " Returns a block buffered writer to stdout for the given color choice."] # [doc = ""] # [doc = " This writer is useful when printing results to a file since it amortizes"] # [doc = " the cost of writing data. The downside of this approach is that it can"] # [doc = " increase the latency of display output when writing to a tty."] # [doc = ""] # [doc = " You might consider using [`stdout`] instead, which chooses the buffering"] # [doc = " strategy automatically based on whether stdout is connected to a tty."] pub fn stdout_buffered_block (color_choice : termcolor :: ColorChoice ,) -> StandardStream { let out = termcolor :: BufferedStandardStream :: stdout (color_choice) ; StandardStream (StandardStreamKind :: BlockBuffered (out)) }
};
}
