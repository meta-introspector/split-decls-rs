// Generated macro for decode_sentence (function)
macro_rules! Depcrate_unicode_sentencedecode_sentence {
() => {
// Module: crate::unicode::sentence
// Provides: {"decode_sentence"}
// Dependencies: {}
fn decode_sentence (bs : & [u8]) -> (& str , usize) { if bs . is_empty () { ("" , 0) } else if let Some (hm) = { let input = Input :: new (bs) . anchored (Anchored :: Yes) ; SENTENCE_BREAK_FWD . try_search_fwd (& input) . unwrap () } { let sentence = unsafe { bs [.. hm . offset ()] . to_str_unchecked () } ; (sentence , sentence . len ()) } else { const INVALID : & str = "\u{FFFD}" ; let (_ , size) = utf8 :: decode_lossy (bs) ; (INVALID , size) } }
};
}
