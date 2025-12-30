// Generated macro for window_size (function)
macro_rules! Depcrate_terminalwindow_size {
() => {
// Module: crate::terminal
// Provides: {"window_size"}
// Dependencies: {}
# [doc = " Returns the terminal size `[WindowSize]`."] # [doc = ""] # [doc = " The width and height in pixels may not be reliably implemented or default to 0."] # [doc = " For unix, <https://man7.org/linux/man-pages/man4/tty_ioctl.4.html> documents them as \"unused\"."] # [doc = " For windows it is not implemented."] pub fn window_size () -> io :: Result < WindowSize > { sys :: window_size () }
};
}
