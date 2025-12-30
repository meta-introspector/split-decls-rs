// Generated macro for impl_42 (impl)
macro_rules! Depcrate_simpleimpl_42 {
() => {
// Module: crate::simple
// Provides: {"impl_42"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser + fmt :: Debug > fmt :: Debug for SimpleHmac < D > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("SimpleHmac { ... }") } }
};
}
