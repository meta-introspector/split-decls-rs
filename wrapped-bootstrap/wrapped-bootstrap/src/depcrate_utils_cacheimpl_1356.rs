// Generated macro for impl_1356 (impl)
macro_rules! Depcrate_utils_cacheimpl_1356 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1356"}
// Dependencies: {}
impl < T : Internable + Hash > Hash for Interned < T > { fn hash < H : Hasher > (& self , state : & mut H) { let l = T :: intern_cache () . lock () . unwrap () ; l . get (* self) . hash (state) } }
};
}
