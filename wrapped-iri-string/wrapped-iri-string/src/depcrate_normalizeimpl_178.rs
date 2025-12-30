// Generated macro for impl_178 (impl)
macro_rules! Depcrate_normalizeimpl_178 {
() => {
// Module: crate::normalize
// Provides: {"impl_178"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Debug for Normalized < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Normalized") . field ("input" , & self . input) . finish () } }
};
}
