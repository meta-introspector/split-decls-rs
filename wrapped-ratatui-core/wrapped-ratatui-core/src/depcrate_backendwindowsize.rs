// Generated macro for WindowSize (struct)
macro_rules! Depcrate_backendWindowSize {
() => {
// Module: crate::backend
// Provides: {"WindowSize"}
// Dependencies: {}
# [doc = " The window size in characters (columns / rows) as well as pixels."] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] pub struct WindowSize { # [doc = " Size of the window in characters (columns / rows)."] pub columns_rows : Size , # [doc = " Size of the window in pixels."] # [doc = ""] # [doc = " The `pixels` fields may not be implemented by all terminals and return `0,0`. See"] # [doc = " <https://man7.org/linux/man-pages/man4/tty_ioctl.4.html> under section \"Get and set window"] # [doc = " size\" / TIOCGWINSZ where the fields are commented as \"unused\"."] pub pixels : Size , }
};
}
