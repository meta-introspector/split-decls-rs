// Generated macro for decode_grapheme (function)
macro_rules! Depcrate_unicode_graphemedecode_grapheme {
() => {
// Module: crate::unicode::grapheme
// Provides: {"decode_grapheme"}
// Dependencies: {}
# [doc = " Decode a grapheme from the given byte string."] # [doc = ""] # [doc = " This returns the resulting grapheme (which may be a Unicode replacement"] # [doc = " codepoint if invalid UTF-8 was found), along with the number of bytes"] # [doc = " decoded in the byte string. The number of bytes decoded may not be the"] # [doc = " same as the length of grapheme in the case where invalid UTF-8 is found."] pub fn decode_grapheme (bs : & [u8]) -> (& str , usize) { if bs . is_empty () { ("" , 0) } else if bs . len () >= 2 && bs [0] . is_ascii () && bs [1] . is_ascii () && ! bs [0] . is_ascii_whitespace () { let grapheme = unsafe { bs [.. 1] . to_str_unchecked () } ; (grapheme , 1) } else if let Some (hm) = { let input = Input :: new (bs) . anchored (Anchored :: Yes) ; GRAPHEME_BREAK_FWD . try_search_fwd (& input) . unwrap () } { let grapheme = unsafe { bs [.. hm . offset ()] . to_str_unchecked () } ; (grapheme , grapheme . len ()) } else { const INVALID : & str = "\u{FFFD}" ; let (_ , size) = utf8 :: decode_lossy (bs) ; (INVALID , size) } }
};
}
