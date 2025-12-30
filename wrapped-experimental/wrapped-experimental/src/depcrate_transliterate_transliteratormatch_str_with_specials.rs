// Generated macro for match_str_with_specials (function)
macro_rules! Depcrate_transliterate_transliteratormatch_str_with_specials {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"match_str_with_specials"}
// Dependencies: {}
# [doc = " Tries to match `query`, which may contain encoded special matchers, on `matcher`. Fills in `match_data` if applicable."] fn match_str_with_specials (query : & str , matcher : & mut impl Utf8Matcher < Forward > , match_data : & mut MatchData , vt : & VarTable ,) -> bool { let query = match find_special (query) { None => { return matcher . match_and_consume_str (query) ; } Some (idx) => { if ! matcher . match_and_consume_str (& query [.. idx]) { return false ; } & query [idx ..] } } ; for query_c in query . chars () { if ! VarTable :: ENCODE_RANGE . contains (& query_c) { if ! matcher . match_and_consume_char (query_c) { return false ; } continue ; } let special_matcher = match vt . lookup_matcher (query_c) { Some (matcher) => matcher , None => { debug_assert ! (false , "invalid encoded special {query_c:?}") ; continue ; } } ; if ! special_matcher . matches (matcher , match_data , vt) { return false ; } } true }
};
}
