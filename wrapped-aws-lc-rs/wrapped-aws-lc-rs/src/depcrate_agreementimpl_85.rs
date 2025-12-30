// Generated macro for impl_85 (impl)
macro_rules! Depcrate_agreementimpl_85 {
() => {
// Module: crate::agreement
// Provides: {"impl_85"}
// Dependencies: {}
impl Debug for AlgorithmID { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { let output = match self { AlgorithmID :: ECDH_P256 => "curve: P256" , AlgorithmID :: ECDH_P384 => "curve: P384" , AlgorithmID :: ECDH_P521 => "curve: P521" , AlgorithmID :: X25519 => "curve: Curve25519" , } ; f . write_str (output) } }
};
}
