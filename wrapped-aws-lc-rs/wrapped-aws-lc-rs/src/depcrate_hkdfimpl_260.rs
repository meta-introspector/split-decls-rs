// Generated macro for impl_260 (impl)
macro_rules! Depcrate_hkdfimpl_260 {
() => {
// Module: crate::hkdf
// Provides: {"impl_260"}
// Dependencies: {}
impl < L : KeyType > fmt :: Debug for Okm < '_ , L > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("hkdf::Okm") . field ("prk" , & self . prk) . finish () } }
};
}
