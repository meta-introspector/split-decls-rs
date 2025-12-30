// Generated macro for set_text_attributes (function)
macro_rules! Depcrate_windows_term_colorsset_text_attributes {
() => {
// Module: crate::windows_term::colors
// Provides: {"set_text_attributes"}
// Dependencies: {}
# [doc = " Set the text attributes of the console represented by the given handle."] # [doc = ""] # [doc = " This corresponds to calling [`SetConsoleTextAttribute`]."] # [doc = ""] # [doc = " [`SetConsoleTextAttribute`]: https://docs.microsoft.com/en-us/windows/console/setconsoletextattribute"] pub (crate) fn set_text_attributes (h : HANDLE , attributes : u16) -> io :: Result < () > { if unsafe { SetConsoleTextAttribute (h , attributes) } == 0 { Err (io :: Error :: last_os_error ()) } else { Ok (()) } }
};
}
