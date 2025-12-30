// Generated macro for store_in_lru_cache (function)
macro_rules! Depcratestore_in_lru_cache {
() => {
// Module: crate
// Provides: {"store_in_lru_cache"}
// Dependencies: {}
pub fn store_in_lru_cache (key : String , value : Vec < u8 >) { if let Ok (mut cache) = LRU_CACHE . lock () { cache . put (key , value) ; } }
};
}
