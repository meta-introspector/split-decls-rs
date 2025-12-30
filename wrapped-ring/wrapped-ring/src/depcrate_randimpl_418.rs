// Generated macro for impl_418 (impl)
macro_rules! Depcrate_randimpl_418 {
() => {
// Module: crate::rand
// Provides: {"impl_418"}
// Dependencies: {}
impl < T > SecureRandom for T where T : sealed :: SecureRandom , { # [inline (always)] fn fill (& self , dest : & mut [u8]) -> Result < () , error :: Unspecified > { self . fill_impl (dest , crate :: sealed :: Arg) } }
};
}
