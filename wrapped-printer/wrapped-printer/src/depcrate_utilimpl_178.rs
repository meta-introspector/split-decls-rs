// Generated macro for impl_178 (impl)
macro_rules! Depcrate_utilimpl_178 {
() => {
// Module: crate::util
// Provides: {"impl_178"}
// Dependencies: {}
impl < M : Matcher > fmt :: Debug for Replacer < M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (dst , matches) = self . replacement () . unwrap_or ((& [] , & [])) ; f . debug_struct ("Replacer") . field ("dst" , & dst) . field ("matches" , & matches) . finish () } }
};
}
