// Generated macro for print_text_without_trailing_newline (function)
macro_rules! Depcrate_text_modificationsprint_text_without_trailing_newline {
() => {
// Module: crate::text_modifications
// Provides: {"print_text_without_trailing_newline"}
// Dependencies: {}
pub (crate) fn print_text_without_trailing_newline < F > (t : & str , f : & mut F , state : & State < '_ >) -> fmt :: Result where F : fmt :: Write , { let line_count = t . split ('\n') . count () ; for (tid , token) in t . split ('\n') . enumerate () { f . write_str (token) ? ; if tid + 1 < line_count { write_padded_newline (f , state) ? ; } } Ok (()) }
};
}
