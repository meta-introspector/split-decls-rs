// Generated macro for tests (module)
macro_rules! Depcrate_registry_cache_controltests {
() => {
// Module: crate::registry::cache_control
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn to_value () { assert_eq ! (CacheControl { public : true , max_age : 0 , } . value () , None) ; assert_eq ! (CacheControl { public : false , max_age : 0 , } . value () , Some ("private" . to_string ())) ; assert_eq ! (CacheControl { public : false , max_age : 10 , } . value () , Some ("max-age=10, private" . to_string ())) ; assert_eq ! (CacheControl { public : true , max_age : 10 , } . value () , Some ("max-age=10" . to_string ())) ; assert_eq ! (CacheControl { public : true , max_age : - 1 , } . value () , Some ("no-cache" . to_string ())) ; assert_eq ! (CacheControl { public : false , max_age : - 1 , } . value () , Some ("no-cache, private" . to_string ())) ; } }
};
}
