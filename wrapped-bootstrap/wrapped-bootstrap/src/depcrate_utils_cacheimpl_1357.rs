// Generated macro for impl_1357 (impl)
macro_rules! Depcrate_utils_cacheimpl_1357 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1357"}
// Dependencies: {}
impl < T : Internable + Deref > Deref for Interned < T > { type Target = T :: Target ; fn deref (& self) -> & Self :: Target { let l = T :: intern_cache () . lock () . unwrap () ; unsafe { mem :: transmute :: < & Self :: Target , & Self :: Target > (l . get (* self)) } } }
};
}
