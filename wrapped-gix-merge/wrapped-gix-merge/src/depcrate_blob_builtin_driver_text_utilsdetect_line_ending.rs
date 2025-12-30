// Generated macro for detect_line_ending (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilsdetect_line_ending {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"detect_line_ending"}
// Dependencies: {}
# [doc = " ## Deviation"] # [doc = ""] # [doc = " This implementation definitely isn't the same as in Git, primarily because it seemed impossible"] # [doc = " to understand what's going on there without investing more time than it seemed worth."] pub fn detect_line_ending (hunks : & [Hunk] , input : & mut InternedInput < & [u8] > , current_tokens : & [Token] ,) -> Option < & 'static BStr > { fn is_eol_crlf (hunks : & [Hunk] , input : & mut InternedInput < & [u8] > , current_tokens : & [Token] ,) -> Option < bool > { let (range , side) = hunks . iter () . rev () . find_map (| h | { (! h . after . is_empty ()) . then_some ((& h . after , h . side)) . or ((! h . before . is_empty ()) . then_some ((& h . before , Side :: Ancestor))) }) ? ; let tokens = tokens_for_side (side , input , current_tokens) ; { let last_line = tokens . get (range . end as usize - 1) . map (| token | & input . interner [* token]) ? ; if last_line . last () == Some (& b'\n') { return last_line . get (last_line . len () . checked_sub (2) ?) . map (| c | * c == b'\r') ; } } let second_to_last_line = tokens . get (range . end . checked_sub (2) ? as usize) . map (| token | & input . interner [* token]) ? ; second_to_last_line . get (second_to_last_line . len () . checked_sub (2) ?) . map (| c | * c == b'\r') } is_eol_crlf (hunks , input , current_tokens) . map (| is_crlf | if is_crlf { b"\r\n" . into () } else { b"\n" . into () }) }
};
}
