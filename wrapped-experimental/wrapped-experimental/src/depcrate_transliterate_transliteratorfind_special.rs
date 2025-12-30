// Generated macro for find_special (function)
macro_rules! Depcrate_transliterate_transliteratorfind_special {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"find_special"}
// Dependencies: {}
# [doc = " Returns the index of the first special construct that is encoded as a private use char in `s`,"] # [doc = " if there is one. Returns `None` if the passed string is pure"] # [doc = " (contains no encoded special constructs)."] fn find_special (s : & str) -> Option < usize > { s . char_indices () . find (| (_ , c) | VarTable :: ENCODE_RANGE . contains (c)) . map (| (i , _) | i) }
};
}
