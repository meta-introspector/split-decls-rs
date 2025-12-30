// Generated macro for Utf8Matcher (trait)
macro_rules! Depcrate_transliterate_transliterator_replaceableUtf8Matcher {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"Utf8Matcher"}
// Dependencies: {}
# [doc = " Matching functionality on strings. Matching can be done in forward or reverse directions, see [`MatchDirection`]."] # [doc = ""] # [doc = " The used indices in method parameters are all compatible with each other."] pub (super) trait Utf8Matcher < D : MatchDirection > : Debug { fn cursor (& self) -> usize ; fn str_range (& self , range : Range < usize >) -> Option < & str > ; fn is_empty (& self) -> bool ; fn match_str (& self , s : & str) -> bool ; fn match_and_consume_str (& mut self , s : & str) -> bool { if self . match_str (s) { self . consume (s . len ()) } else { false } } fn match_and_consume_char (& mut self , c : char) -> bool { self . match_and_consume_str (c . encode_utf8 (& mut [0 ; 4])) } fn match_start_anchor (& self) -> bool ; fn match_end_anchor (& self) -> bool ; fn consume (& mut self , len : usize) -> bool ; fn next_char (& self) -> Option < char > ; }
};
}
