// Generated macro for impl_40 (impl)
macro_rules! Depcrate_byteimpl_40 {
() => {
// Module: crate::byte
// Provides: {"impl_40"}
// Dependencies: {}
impl < B : Buffer > fmt :: Display for ByteLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
};
}
