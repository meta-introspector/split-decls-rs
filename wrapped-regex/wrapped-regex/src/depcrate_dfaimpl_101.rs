// Generated macro for impl_101 (impl)
macro_rules! Depcrate_dfaimpl_101 {
() => {
// Module: crate::dfa
// Provides: {"impl_101"}
// Dependencies: {}
impl CacheInner { # [doc = " Resets the cache size to account for fixed costs, such as the program"] # [doc = " and stack sizes."] fn reset_size (& mut self) { self . size = (self . start_states . len () * mem :: size_of :: < StatePtr > ()) + (self . stack . len () * mem :: size_of :: < InstPtr > ()) ; } }
};
}
