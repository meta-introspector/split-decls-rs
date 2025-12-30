// Generated macro for stdout_buffered_line (function)
macro_rules! Depcrate_wtrstdout_buffered_line {
() => {
// Module: crate::wtr
// Provides: {"stdout_buffered_line"}
// Dependencies: {}
# [doc = " Returns a line buffered writer to stdout for the given color choice."] # [doc = ""] # [doc = " This writer is useful when printing results directly to a tty such that"] # [doc = " users see output as soon as it's written. The downside of this approach"] # [doc = " is that it can be slower, especially when there is a lot of output."] # [doc = ""] # [doc = " You might consider using [`stdout`] instead, which chooses the buffering"] # [doc = " strategy automatically based on whether stdout is connected to a tty."] pub fn stdout_buffered_line (color_choice : termcolor :: ColorChoice ,) -> StandardStream { let out = termcolor :: StandardStream :: stdout (color_choice) ; StandardStream (StandardStreamKind :: LineBuffered (out)) }
};
}
