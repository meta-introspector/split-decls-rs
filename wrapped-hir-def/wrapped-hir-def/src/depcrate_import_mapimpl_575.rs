// Generated macro for impl_575 (impl)
macro_rules! Depcrate_import_mapimpl_575 {
() => {
// Module: crate::import_map
// Provides: {"impl_575"}
// Dependencies: {}
impl SearchMode { pub fn check (self , query : & str , case_sensitive : bool , candidate : & str) -> bool { match self { SearchMode :: Exact if case_sensitive => candidate == query , SearchMode :: Exact => candidate . eq_ignore_ascii_case (query) , SearchMode :: Prefix => { query . len () <= candidate . len () && { let prefix = & candidate [.. query . len ()] ; if case_sensitive { prefix == query } else { prefix . eq_ignore_ascii_case (query) } } } SearchMode :: Fuzzy => { let mut name = candidate ; query . chars () . all (| query_char | { let m = if case_sensitive { name . match_indices (query_char) . next () } else { name . match_indices ([query_char , query_char . to_ascii_uppercase ()]) . next () } ; match m { Some ((index , _)) => { name = name [index ..] . strip_prefix (| _ : char | true) . unwrap_or_default () ; true } None => false , } }) } } } }
};
}
