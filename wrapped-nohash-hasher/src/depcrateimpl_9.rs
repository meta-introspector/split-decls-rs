// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < T > Clone for NoHashHasher < T > { # [cfg (debug_assertions)] fn clone (& self) -> Self { NoHashHasher (self . 0 , self . 1 , self . 2) } # [cfg (not (debug_assertions))] fn clone (& self) -> Self { NoHashHasher (self . 0 , self . 1) } }
};
}
