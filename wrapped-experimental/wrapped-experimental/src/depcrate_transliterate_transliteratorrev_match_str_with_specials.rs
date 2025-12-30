// Generated macro for rev_match_str_with_specials (function)
macro_rules! Depcrate_transliterate_transliteratorrev_match_str_with_specials {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"rev_match_str_with_specials"}
// Dependencies: {}
# [doc = " Tries to match `query`, which may contain encoded special matchers, on `matcher` from the right. Fills in `match_data` if applicable."] fn rev_match_str_with_specials (query : & str , matcher : & mut impl Utf8Matcher < Reverse > , match_data : & mut MatchData , vt : & VarTable ,) -> bool { let query = match rev_find_special (query) { None => { return matcher . match_and_consume_str (query) ; } Some (idx) => { if ! matcher . match_and_consume_str (& query [idx ..]) { return false ; } & query [.. idx] } } ; for query_c in query . chars () . rev () { if ! VarTable :: ENCODE_RANGE . contains (& query_c) { if ! matcher . match_and_consume_char (query_c) { return false ; } continue ; } let special_matcher = match vt . lookup_matcher (query_c) { Some (matcher) => matcher , None => { debug_assert ! (false , "invalid encoded special {query_c:?}") ; continue ; } } ; if ! special_matcher . rev_matches (matcher , match_data , vt) { return false ; } } true }
};
}
