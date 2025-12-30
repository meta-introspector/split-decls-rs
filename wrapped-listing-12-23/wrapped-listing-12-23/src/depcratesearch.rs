// Generated macro for search (function)
macro_rules! Depcratesearch {
() => {
// Module: crate
// Provides: {"search"}
// Dependencies: {}
pub fn search < 'a > (query : & str , contents : & 'a str) -> Vec < & 'a str > { let mut results = Vec :: new () ; for line in contents . lines () { if line . contains (query) { results . push (line) ; } } results }
};
}
