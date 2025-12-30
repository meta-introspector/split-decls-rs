// Generated macro for impl_659 (impl)
macro_rules! Depcrate_ed25519impl_659 {
() => {
// Module: crate::ed25519
// Provides: {"impl_659"}
// Dependencies: {}
impl Debug for Ed25519KeyPair { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (& format ! ("Ed25519KeyPair {{ public_key: PublicKey(\"{}\") }}" , hex :: encode (& self . public_key))) } }
};
}
