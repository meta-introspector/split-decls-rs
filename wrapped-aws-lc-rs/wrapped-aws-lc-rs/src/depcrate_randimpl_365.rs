// Generated macro for impl_365 (impl)
macro_rules! Depcrate_randimpl_365 {
() => {
// Module: crate::rand
// Provides: {"impl_365"}
// Dependencies: {}
impl < T > SecureRandom for T where T : sealed :: SecureRandom , { # [inline] fn fill (& self , dest : & mut [u8]) -> Result < () , Unspecified > { self . fill_impl (dest) } }
};
}
