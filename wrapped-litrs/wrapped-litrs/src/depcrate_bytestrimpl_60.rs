// Generated macro for impl_60 (impl)
macro_rules! Depcrate_bytestrimpl_60 {
() => {
// Module: crate::bytestr
// Provides: {"impl_60"}
// Dependencies: {}
impl < B : Buffer > fmt :: Display for ByteStringLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
};
}
