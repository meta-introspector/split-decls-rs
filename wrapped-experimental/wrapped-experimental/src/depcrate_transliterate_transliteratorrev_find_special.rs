// Generated macro for rev_find_special (function)
macro_rules! Depcrate_transliterate_transliteratorrev_find_special {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"rev_find_special"}
// Dependencies: {}
# [doc = " Returns the index of the char to the right of the first (from the right) special construct"] # [doc = " encoded as a private use char. Returns `None` if the passed string is pure"] # [doc = " (contains no encoded special constructs)."] fn rev_find_special (s : & str) -> Option < usize > { s . char_indices () . rfind (| (_ , c) | VarTable :: ENCODE_RANGE . contains (c)) . map (| (i , c) | i + c . len_utf8 ()) }
};
}
