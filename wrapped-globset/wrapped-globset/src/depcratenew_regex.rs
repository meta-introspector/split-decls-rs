// Generated macro for new_regex (function)
macro_rules! Depcratenew_regex {
() => {
// Module: crate
// Provides: {"new_regex"}
// Dependencies: {}
fn new_regex (pat : & str) -> Result < Regex , Error > { let syntax = regex_automata :: util :: syntax :: Config :: new () . utf8 (false) . dot_matches_new_line (true) ; let config = Regex :: config () . utf8_empty (false) . nfa_size_limit (Some (10 * (1 << 20))) . hybrid_cache_capacity (10 * (1 << 20)) ; Regex :: builder () . syntax (syntax) . configure (config) . build (pat) . map_err (| err | Error { glob : Some (pat . to_string ()) , kind : ErrorKind :: Regex (err . to_string ()) , } ,) }
};
}
