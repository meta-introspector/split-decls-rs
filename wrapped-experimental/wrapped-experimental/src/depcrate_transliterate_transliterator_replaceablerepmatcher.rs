// Generated macro for RepMatcher (struct)
macro_rules! Depcrate_transliterate_transliterator_replaceableRepMatcher {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"RepMatcher"}
// Dependencies: {}
# [doc = " Supports safe conversion rule matching over a [`Replaceable`]."] # [doc = ""] # [doc = " Conversion rule matching consists of three parts:"] # [doc = " 1. Matching the ante. This is a right-aligned (reverse) match at the `Replaceable`'s cursor."] # [doc = " 2. Matching the key. This is a left-aligned (forward) match at the `Replaceable`'s cursor."] # [doc = " 3. Matching the post. This is a left-aligned (forward) match at the end of _the matched key_."] # [doc = "    Note that this means we _cannot_ match the key after the post, as the post match is aligned"] # [doc = "    to the end of the key match. The `KEY_FINISHED` parameter enforces this at compile-time."] # [doc = ""] # [doc = " After a successful match, the replacement can be applied using the [`Insertable`] returned by"] # [doc = " [`RepMatcher::finish_match`]."] # [doc = ""] # [doc = " # Safety"] # [doc = " The matched portions of the string (as defined by `rep.cursor`, `key_match_len`,"] # [doc = " `ante_match_len` and `post_match_len`) are all guaranteed to be valid UTF-8 subslices of"] # [doc = " the `Replaceable`'s internal text."] # [doc = ""] # [doc = " The `RepMatcher` does not modify the contained `Replaceable`."] # [derive (Debug)] pub (super) struct RepMatcher < 'a , 'b , const KEY_FINISHED : bool > { rep : & 'b mut Replaceable < 'a > , key_match_len : usize , ante_match_len : usize , post_match_len : usize , forward_cursor : usize , }
};
}
