// Generated macro for str_eq (function)
macro_rules! Depcrate_cmpstr_eq {
() => {
// Module: crate::cmp
// Provides: {"str_eq"}
// Dependencies: {}
pub (crate) fn str_eq (mime : & Mime , s : & str) -> bool { if mime . has_params () { Parser :: can_range () . parse (s) . map (| other_mime | { mime_eq (mime , & other_mime) }) . unwrap_or (false) } else { mime . as_ref () . eq_ignore_ascii_case (s) } }
};
}
