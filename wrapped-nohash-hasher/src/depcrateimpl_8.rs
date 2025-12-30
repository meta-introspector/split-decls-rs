// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T > Default for NoHashHasher < T > { # [cfg (debug_assertions)] fn default () -> Self { NoHashHasher (0 , false , PhantomData) } # [cfg (not (debug_assertions))] fn default () -> Self { NoHashHasher (0 , PhantomData) } }
};
}
