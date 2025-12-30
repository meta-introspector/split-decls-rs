// Generated macro for decode_last_grapheme (function)
macro_rules! Depcrate_unicode_graphemedecode_last_grapheme {
() => {
// Module: crate::unicode::grapheme
// Provides: {"decode_last_grapheme"}
// Dependencies: {}
fn decode_last_grapheme (bs : & [u8]) -> (& str , usize) { if bs . is_empty () { ("" , 0) } else if let Some (hm) = { let input = Input :: new (bs) . anchored (Anchored :: Yes) ; GRAPHEME_BREAK_REV . try_search_rev (& input) . unwrap () } { let start = adjust_rev_for_regional_indicator (bs , hm . offset ()) ; let grapheme = unsafe { bs [start ..] . to_str_unchecked () } ; (grapheme , grapheme . len ()) } else { const INVALID : & str = "\u{FFFD}" ; let (_ , size) = utf8 :: decode_last_lossy (bs) ; (INVALID , size) } }
};
}
