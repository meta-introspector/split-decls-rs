// Generated macro for sanitize_sh (function)
macro_rules! Depcratesanitize_sh {
() => {
// Module: crate
// Provides: {"sanitize_sh"}
// Dependencies: {}
fn sanitize_sh (path : & Path) -> String { if ! cfg ! (windows) { return path . to_string_lossy () . into_owned () ; } let path = path . to_string_lossy () . replace ("\\" , "/") ; return change_drive (& path) . unwrap_or (path) ; fn change_drive (s : & str) -> Option < String > { let mut ch = s . chars () ; let drive = ch . next () . unwrap_or ('C') ; if ch . next () != Some (':') { return None ; } if ch . next () != Some ('/') { return None ; } Some (format ! ("/{}/{}" , drive , & s [drive . len_utf8 () + 2 ..])) } }
};
}
