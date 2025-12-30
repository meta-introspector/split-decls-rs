// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1030 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1030"}
// Dependencies: {}
impl < const KEY_FINISHED : bool > Utf8Matcher < Forward > for RepMatcher < '_ , '_ , KEY_FINISHED > { fn cursor (& self) -> usize { self . forward_cursor } fn str_range (& self , range : Range < usize >) -> Option < & str > { self . rep . as_str () . get (range) } fn is_empty (& self) -> bool { self . remaining () == 0 } fn match_str (& self , s : & str) -> bool { self . remaining_forward_slice () . starts_with (s) } fn match_start_anchor (& self) -> bool { self . forward_cursor == 0 } fn match_end_anchor (& self) -> bool { self . forward_cursor == self . rep . content . len () } fn consume (& mut self , len : usize) -> bool { if len <= self . remaining () { assert ! (self . remaining_forward_slice () . is_char_boundary (len)) ; if KEY_FINISHED { self . post_match_len += len ; } else { self . key_match_len += len ; } self . forward_cursor += len ; true } else { false } } fn next_char (& self) -> Option < char > { self . remaining_forward_slice () . chars () . next () } }
};
}
