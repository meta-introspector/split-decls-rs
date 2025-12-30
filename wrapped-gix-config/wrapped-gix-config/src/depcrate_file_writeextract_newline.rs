// Generated macro for extract_newline (function)
macro_rules! Depcrate_file_writeextract_newline {
() => {
// Module: crate::file::write
// Provides: {"extract_newline"}
// Dependencies: {}
pub (crate) fn extract_newline < 'a > (e : & 'a Event < '_ >) -> Option < & 'a BStr > { Some (match e { Event :: Newline (b) => { let nl = b . as_ref () ; if nl . contains (& b'\r') { "\r\n" . into () } else { "\n" . into () } } _ => return None , }) }
};
}
