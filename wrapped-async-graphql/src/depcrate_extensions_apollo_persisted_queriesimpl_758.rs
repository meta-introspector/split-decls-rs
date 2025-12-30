// Generated macro for impl_758 (impl)
macro_rules! Depcrate_extensions_apollo_persisted_queriesimpl_758 {
() => {
// Module: crate::extensions::apollo_persisted_queries
// Provides: {"impl_758"}
// Dependencies: {}
# [async_trait :: async_trait] impl CacheStorage for LruCacheStorage { async fn get (& self , key : String) -> Option < ExecutableDocument > { let mut cache = self . 0 . lock () . await ; cache . get (& key) . cloned () } async fn set (& self , key : String , query : ExecutableDocument) { let mut cache = self . 0 . lock () . await ; cache . put (key , query) ; } }
};
}
