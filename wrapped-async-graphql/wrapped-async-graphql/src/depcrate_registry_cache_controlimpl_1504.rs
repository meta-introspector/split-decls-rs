// Generated macro for impl_1504 (impl)
macro_rules! Depcrate_registry_cache_controlimpl_1504 {
() => {
// Module: crate::registry::cache_control
// Provides: {"impl_1504"}
// Dependencies: {}
impl CacheControl { # [must_use] pub (crate) fn merge (self , other : & CacheControl) -> CacheControl { CacheControl { public : self . public && other . public , max_age : match (self . max_age , other . max_age) { (- 1 , _) => - 1 , (_ , - 1) => - 1 , (a , 0) => a , (0 , b) => b , (a , b) => a . min (b) , } , } } }
};
}
