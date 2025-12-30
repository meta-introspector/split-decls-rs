// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1031 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1031"}
// Dependencies: {}
impl < const KEY_FINISHED : bool > Utf8Matcher < Reverse > for RepMatcher < '_ , '_ , KEY_FINISHED > { fn cursor (& self) -> usize { self . ante_cursor () } fn str_range (& self , range : Range < usize >) -> Option < & str > { self . rep . as_str () . get (range) } fn is_empty (& self) -> bool { self . ante_cursor () == 0 } fn match_str (& self , s : & str) -> bool { self . remaining_ante_slice () . ends_with (s) } fn match_start_anchor (& self) -> bool { self . ante_cursor () == 0 } fn match_end_anchor (& self) -> bool { self . ante_cursor () == self . rep . content . len () } fn consume (& mut self , len : usize) -> bool { if len <= self . ante_cursor () { assert ! (self . remaining_ante_slice () . is_char_boundary (self . ante_cursor () - len)) ; self . ante_match_len += len ; true } else { false } } fn next_char (& self) -> Option < char > { self . remaining_ante_slice () . chars () . next_back () } }
};
}
