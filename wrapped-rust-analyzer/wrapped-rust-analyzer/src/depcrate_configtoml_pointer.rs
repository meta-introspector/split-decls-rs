// Generated macro for toml_pointer (function)
macro_rules! Depcrate_configtoml_pointer {
() => {
// Module: crate::config
// Provides: {"toml_pointer"}
// Dependencies: {}
fn toml_pointer < 'a > (toml : & 'a toml :: Table , pointer : & str) -> Option < & 'a toml :: Value > { fn parse_index (s : & str) -> Option < usize > { if s . starts_with ('+') || (s . starts_with ('0') && s . len () != 1) { return None ; } s . parse () . ok () } if pointer . is_empty () { return None ; } if ! pointer . starts_with ('/') { return None ; } let mut parts = pointer . split ('/') . skip (1) ; let first = parts . next () ? ; let init = toml . get (first) ? ; parts . map (| x | x . replace ("~1" , "/") . replace ("~0" , "~")) . try_fold (init , | target , token | { match target { toml :: Value :: Table (table) => table . get (& token) , toml :: Value :: Array (list) => parse_index (& token) . and_then (move | x | list . get (x)) , _ => None , } }) }
};
}
