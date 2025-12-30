// Generated macro for replace_str_with_specials (function)
macro_rules! Depcrate_transliterate_transliteratorreplace_str_with_specials {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"replace_str_with_specials"}
// Dependencies: {}
# [doc = " Applies the replacements from the `replacement`, which may contain encoded special replacers, to `dest`,"] # [doc = " including non-default cursor updates."] fn replace_str_with_specials (replacement : & str , dest : & mut Insertable , data : & MatchData , vt : & VarTable , env : & Env ,) { let replacement = match find_special (replacement) { None => { dest . push_str (replacement) ; return ; } Some (idx) => { dest . push_str (& replacement [.. idx]) ; & replacement [idx ..] } } ; for rep_c in replacement . chars () { if ! VarTable :: ENCODE_RANGE . contains (& rep_c) { dest . push (rep_c) ; continue ; } let replacer = match vt . lookup_replacer (rep_c) { Some (replacer) => replacer , None => { debug_assert ! (false , "invalid encoded special {rep_c:?}") ; continue ; } } ; replacer . replace (dest , data , vt , env) ; } }
};
}
