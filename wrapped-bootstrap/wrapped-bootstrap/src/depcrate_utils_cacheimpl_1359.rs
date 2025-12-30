// Generated macro for impl_1359 (impl)
macro_rules! Depcrate_utils_cacheimpl_1359 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1359"}
// Dependencies: {}
impl < T : Internable + PartialOrd > PartialOrd for Interned < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { let l = T :: intern_cache () . lock () . unwrap () ; l . get (* self) . partial_cmp (l . get (* other)) } }
};
}
