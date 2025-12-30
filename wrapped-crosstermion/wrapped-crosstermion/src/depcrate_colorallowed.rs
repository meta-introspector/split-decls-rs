// Generated macro for allowed (function)
macro_rules! Depcrate_colorallowed {
() => {
// Module: crate::color
// Provides: {"allowed"}
// Dependencies: {}
# [doc = " Return true if we should colorize the output, based on [clicolors spec](https://bixense.com/clicolors/) and [no-color spec](https://no-color.org)"] # [doc = ""] # [doc = " Note that you should also validate that the output stream is actually connected to a terminal, which usually looks like"] # [doc = " `atty::is(atty::Stream::Stdout) && should_colorize()"] pub fn allowed () -> bool { allow_clicolors_spec () && allow_by_no_color_spec () }
};
}
