// Generated macro for CacheStorage (trait)
macro_rules! Depcrate_extensions_apollo_persisted_queriesCacheStorage {
() => {
// Module: crate::extensions::apollo_persisted_queries
// Provides: {"CacheStorage"}
// Dependencies: {}
# [doc = " Cache storage for persisted queries."] # [async_trait :: async_trait] pub trait CacheStorage : Send + Sync + Clone + 'static { # [doc = " Load the query by `key`."] async fn get (& self , key : String) -> Option < ExecutableDocument > ; # [doc = " Save the query by `key`."] async fn set (& self , key : String , query : ExecutableDocument) ; }
};
}
