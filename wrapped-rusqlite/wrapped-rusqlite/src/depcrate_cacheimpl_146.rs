// Generated macro for impl_146 (impl)
macro_rules! Depcrate_cacheimpl_146 {
() => {
// Module: crate::cache
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'conn > DerefMut for CachedStatement < 'conn > { # [inline] fn deref_mut (& mut self) -> & mut Statement < 'conn > { self . stmt . as_mut () . unwrap () } }
};
}
