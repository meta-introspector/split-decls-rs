// Generated macro for tests (module)
macro_rules! Depcrate_extensions_apollo_persisted_queriestests {
() => {
// Module: crate::extensions::apollo_persisted_queries
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [tokio :: test] async fn test () { use super :: * ; use crate :: * ; struct Query ; # [Object (internal)] impl Query { async fn value (& self) -> i32 { 100 } } let schema = Schema :: build (Query , EmptyMutation , EmptySubscription) . extension (ApolloPersistedQueries :: new (LruCacheStorage :: new (256))) . finish () ; let mut request = Request :: new ("{ value }") ; request . extensions . insert ("persistedQuery" . to_string () , value ! ({ "version" : 1 , "sha256Hash" : "854174ebed716fe24fd6659c30290aecd9bc1d17dc4f47939a1848a1b8ed3c6b" , }) ,) ; assert_eq ! (schema . execute (request) . await . into_result () . unwrap () . data , value ! ({ "value" : 100 })) ; let mut request = Request :: new ("") ; request . extensions . insert ("persistedQuery" . to_string () , value ! ({ "version" : 1 , "sha256Hash" : "854174ebed716fe24fd6659c30290aecd9bc1d17dc4f47939a1848a1b8ed3c6b" , }) ,) ; assert_eq ! (schema . execute (request) . await . into_result () . unwrap () . data , value ! ({ "value" : 100 })) ; let mut request = Request :: new ("") ; request . extensions . insert ("persistedQuery" . to_string () , value ! ({ "version" : 1 , "sha256Hash" : "def" , }) ,) ; assert_eq ! (schema . execute (request) . await . into_result () . unwrap_err () , vec ! [ServerError :: new ("PersistedQueryNotFound" , None)]) ; } }
};
}
