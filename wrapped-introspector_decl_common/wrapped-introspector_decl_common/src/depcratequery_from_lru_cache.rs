// Generated macro for query_from_lru_cache (function)
macro_rules! Depcratequery_from_lru_cache {
() => {
// Module: crate
// Provides: {"query_from_lru_cache"}
// Dependencies: {}
pub fn query_from_lru_cache (key : & str) -> Option < Vec < u8 > > { if let Ok (mut cache) = LRU_CACHE . lock () { cache . get (key) . cloned () } else { None } }
};
}
