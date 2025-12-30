// Generated macro for impl_1346 (impl)
macro_rules! Depcrate_utils_cacheimpl_1346 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1346"}
// Dependencies: {}
impl < T : Internable + Default > Default for Interned < T > { fn default () -> Self { T :: default () . intern () } }
};
}
