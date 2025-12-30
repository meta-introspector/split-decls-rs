// Generated macro for impl_151 (impl)
macro_rules! Depcrate_packed_patternimpl_151 {
() => {
// Module: crate::packed::pattern
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Pattern < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Pattern") . field ("lit" , & String :: from_utf8_lossy (self . 0)) . finish () } }
};
}
