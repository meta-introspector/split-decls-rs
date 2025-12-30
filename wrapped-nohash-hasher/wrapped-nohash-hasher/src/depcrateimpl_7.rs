// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < T > fmt :: Debug for NoHashHasher < T > { # [cfg (debug_assertions)] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("NoHashHasher") . field (& self . 0) . field (& self . 1) . finish () } # [cfg (not (debug_assertions))] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("NoHashHasher") . field (& self . 0) . finish () } }
};
}
