// Generated macro for new_regex_set (function)
macro_rules! Depcratenew_regex_set {
() => {
// Module: crate
// Provides: {"new_regex_set"}
// Dependencies: {}
fn new_regex_set (pats : Vec < String >) -> Result < Regex , Error > { let syntax = regex_automata :: util :: syntax :: Config :: new () . utf8 (false) . dot_matches_new_line (true) ; let config = Regex :: config () . match_kind (regex_automata :: MatchKind :: All) . utf8_empty (false) . nfa_size_limit (Some (10 * (1 << 20))) . hybrid_cache_capacity (10 * (1 << 20)) ; Regex :: builder () . syntax (syntax) . configure (config) . build_many (& pats) . map_err (| err | Error { glob : None , kind : ErrorKind :: Regex (err . to_string ()) , }) }
};
}
