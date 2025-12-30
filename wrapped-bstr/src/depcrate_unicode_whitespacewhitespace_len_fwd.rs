// Generated macro for whitespace_len_fwd (function)
macro_rules! Depcrate_unicode_whitespacewhitespace_len_fwd {
() => {
// Module: crate::unicode::whitespace
// Provides: {"whitespace_len_fwd"}
// Dependencies: {}
# [doc = " Return the first position of a non-whitespace character."] pub fn whitespace_len_fwd (slice : & [u8]) -> usize { let input = Input :: new (slice) . anchored (Anchored :: Yes) ; WHITESPACE_ANCHORED_FWD . try_search_fwd (& input) . unwrap () . map_or (0 , | hm | hm . offset ()) }
};
}
