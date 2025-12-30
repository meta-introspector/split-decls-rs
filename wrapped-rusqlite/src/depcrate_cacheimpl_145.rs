// Generated macro for impl_145 (impl)
macro_rules! Depcrate_cacheimpl_145 {
() => {
// Module: crate::cache
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'conn > Deref for CachedStatement < 'conn > { type Target = Statement < 'conn > ; # [inline] fn deref (& self) -> & Statement < 'conn > { self . stmt . as_ref () . unwrap () } }
};
}
