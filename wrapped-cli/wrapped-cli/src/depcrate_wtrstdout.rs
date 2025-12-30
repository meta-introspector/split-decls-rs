// Generated macro for stdout (function)
macro_rules! Depcrate_wtrstdout {
() => {
// Module: crate::wtr
// Provides: {"stdout"}
// Dependencies: {}
# [doc = " Returns a possibly buffered writer to stdout for the given color choice."] # [doc = ""] # [doc = " The writer returned is either line buffered or block buffered. The decision"] # [doc = " between these two is made automatically based on whether a tty is attached"] # [doc = " to stdout or not. If a tty is attached, then line buffering is used."] # [doc = " Otherwise, block buffering is used. In general, block buffering is more"] # [doc = " efficient, but may increase the time it takes for the end user to see the"] # [doc = " first bits of output."] # [doc = ""] # [doc = " If you need more fine grained control over the buffering mode, then use one"] # [doc = " of `stdout_buffered_line` or `stdout_buffered_block`."] # [doc = ""] # [doc = " The color choice given is passed along to the underlying writer. To"] # [doc = " completely disable colors in all cases, use `ColorChoice::Never`."] pub fn stdout (color_choice : termcolor :: ColorChoice) -> StandardStream { if std :: io :: stdout () . is_terminal () { stdout_buffered_line (color_choice) } else { stdout_buffered_block (color_choice) } }
};
}
