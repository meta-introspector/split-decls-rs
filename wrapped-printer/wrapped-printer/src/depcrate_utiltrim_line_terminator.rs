// Generated macro for trim_line_terminator (function)
macro_rules! Depcrate_utiltrim_line_terminator {
() => {
// Module: crate::util
// Provides: {"trim_line_terminator"}
// Dependencies: {}
# [doc = " Given a buf and some bounds, if there is a line terminator at the end of"] # [doc = " the given bounds in buf, then the bounds are trimmed to remove the line"] # [doc = " terminator, returning the slice of the removed line terminator (if any)."] pub (crate) fn trim_line_terminator < 'b > (searcher : & Searcher , buf : & 'b [u8] , line : & mut Match ,) -> & 'b [u8] { let lineterm = searcher . line_terminator () ; if lineterm . is_suffix (& buf [* line]) { let mut end = line . end () - 1 ; if lineterm . is_crlf () && end > 0 && buf . get (end - 1) == Some (& b'\r') { end -= 1 ; } let orig_end = line . end () ; * line = line . with_end (end) ; & buf [end .. orig_end] } else { & [] } }
};
}
