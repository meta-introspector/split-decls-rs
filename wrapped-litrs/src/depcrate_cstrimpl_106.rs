// Generated macro for impl_106 (impl)
macro_rules! Depcrate_cstrimpl_106 {
() => {
// Module: crate::cstr
// Provides: {"impl_106"}
// Dependencies: {}
impl < B : Buffer > fmt :: Display for CStringLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
};
}
