// Generated macro for impl_1360 (impl)
macro_rules! Depcrate_utils_cacheimpl_1360 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1360"}
// Dependencies: {}
impl < T : Internable + Ord > Ord for Interned < T > { fn cmp (& self , other : & Self) -> Ordering { let l = T :: intern_cache () . lock () . unwrap () ; l . get (* self) . cmp (l . get (* other)) } }
};
}
