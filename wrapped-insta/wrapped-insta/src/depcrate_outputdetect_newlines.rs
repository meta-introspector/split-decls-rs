// Generated macro for detect_newlines (function)
macro_rules! Depcrate_outputdetect_newlines {
() => {
// Module: crate::output
// Provides: {"detect_newlines"}
// Dependencies: {}
fn detect_newlines (s : & str) -> (bool , bool , bool) { let mut last_char = None ; let mut detected_crlf = false ; let mut detected_cr = false ; let mut detected_lf = false ; for c in s . chars () { if c == '\n' { if last_char . take () == Some ('\r') { detected_crlf = true ; } else { detected_lf = true ; } } if last_char == Some ('\r') { detected_cr = true ; } last_char = Some (c) ; } if last_char == Some ('\r') { detected_cr = true ; } (detected_cr , detected_crlf , detected_lf) }
};
}
