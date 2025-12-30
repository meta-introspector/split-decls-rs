// Generated macro for estimate_replacement_size (function)
macro_rules! Depcrate_transliterate_transliteratorestimate_replacement_size {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"estimate_replacement_size"}
// Dependencies: {}
# [doc = " Recursively estimates the size of the replacement string."] fn estimate_replacement_size (replacement : & str , data : & MatchData , vt : & VarTable) -> usize { let mut size ; let replacement_tail ; match find_special (replacement) { None => return replacement . len () , Some (idx) => { size = idx ; replacement_tail = & replacement [idx ..] ; } } for rep_c in replacement_tail . chars () { if ! VarTable :: ENCODE_RANGE . contains (& rep_c) { size += rep_c . len_utf8 () ; continue ; } let replacer = match vt . lookup_replacer (rep_c) { Some (replacer) => replacer , None => { debug_assert ! (false , "invalid encoded special {rep_c:?}") ; continue ; } } ; size += replacer . estimate_size (data , vt) ; } size }
};
}
