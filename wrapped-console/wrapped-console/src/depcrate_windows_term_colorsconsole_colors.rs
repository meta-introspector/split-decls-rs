// Generated macro for console_colors (function)
macro_rules! Depcrate_windows_term_colorsconsole_colors {
() => {
// Module: crate::windows_term::colors
// Provides: {"console_colors"}
// Dependencies: {}
pub (crate) fn console_colors (out : & Term , mut con : Console , bytes : & [u8]) -> io :: Result < () > { let s = from_utf8 (bytes) . expect ("data to be printed is not an ansi string") ; let mut iter = AnsiCodeIterator :: new (s) ; while ! iter . rest_slice () . is_empty () { if let Some ((part , is_esc)) = iter . next () { if ! is_esc { out . write_through_common (part . as_bytes ()) ? ; } else if part == "\x1b[0m" { con . reset () ? ; } else if let Some ((intense , color , fg_bg)) = driver (parse_color , part) { match fg_bg { FgBg :: Foreground => con . fg (intense , color) , FgBg :: Background => con . bg (intense , color) , } ? ; } else if driver (parse_attr , part) . is_none () { out . write_through_common (part . as_bytes ()) ? ; } } } Ok (()) }
};
}
