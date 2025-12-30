// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1028 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1028"}
// Dependencies: {}
impl < 'a , 'b > RepMatcher < 'a , 'b , false > { pub (super) fn finish_match (self) -> Insertable < 'a , 'b > { Insertable :: from_matcher (self . finish_key ()) } pub (super) fn finish_key (self) -> RepMatcher < 'a , 'b , true > { RepMatcher { rep : self . rep , key_match_len : self . key_match_len , ante_match_len : self . ante_match_len , post_match_len : self . post_match_len , forward_cursor : self . forward_cursor , } } }
};
}
