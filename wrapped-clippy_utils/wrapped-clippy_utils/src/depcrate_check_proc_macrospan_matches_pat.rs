// Generated macro for span_matches_pat (function)
macro_rules! Depcrate_check_proc_macrospan_matches_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"span_matches_pat"}
// Dependencies: {}
# [doc = " Checks if the start and the end of the span's text matches the patterns. This will return false"] # [doc = " if the span crosses multiple files or if source is not available."] fn span_matches_pat (sess : & Session , span : Span , start_pat : Pat , end_pat : Pat) -> bool { let pos = sess . source_map () . lookup_byte_offset (span . lo ()) ; let Some (ref src) = pos . sf . src else { return false ; } ; let end = span . hi () - pos . sf . start_pos ; src . get (pos . pos . 0 as usize .. end . 0 as usize) . is_some_and (| s | { let start_str = s . trim_start_matches (| c : char | c . is_whitespace () || c == '(') ; let end_str = s . trim_end_matches (| c : char | c . is_whitespace () || c == ')' || c == ',') ; (match start_pat { Pat :: Str (text) => start_str . starts_with (text) , Pat :: MultiStr (texts) => texts . iter () . any (| s | start_str . starts_with (s)) , Pat :: OwnedMultiStr (texts) => texts . iter () . any (| s | start_str . starts_with (s)) , Pat :: Sym (sym) => start_str . starts_with (sym . as_str ()) , Pat :: Num => start_str . as_bytes () . first () . is_some_and (u8 :: is_ascii_digit) , } && match end_pat { Pat :: Str (text) => end_str . ends_with (text) , Pat :: MultiStr (texts) => texts . iter () . any (| s | end_str . ends_with (s)) , Pat :: OwnedMultiStr (texts) => texts . iter () . any (| s | end_str . ends_with (s)) , Pat :: Sym (sym) => end_str . ends_with (sym . as_str ()) , Pat :: Num => end_str . as_bytes () . last () . is_some_and (u8 :: is_ascii_hexdigit) , }) }) }
};
}
