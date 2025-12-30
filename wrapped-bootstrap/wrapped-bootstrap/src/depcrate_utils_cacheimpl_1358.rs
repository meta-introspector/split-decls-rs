// Generated macro for impl_1358 (impl)
macro_rules! Depcrate_utils_cacheimpl_1358 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1358"}
// Dependencies: {}
impl < T : Internable + AsRef < U > , U : ? Sized > AsRef < U > for Interned < T > { fn as_ref (& self) -> & U { let l = T :: intern_cache () . lock () . unwrap () ; unsafe { mem :: transmute :: < & U , & U > (l . get (* self) . as_ref ()) } } }
};
}
