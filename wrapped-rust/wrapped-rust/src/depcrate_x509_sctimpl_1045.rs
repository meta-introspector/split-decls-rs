// Generated macro for impl_1045 (impl)
macro_rules! Depcrate_x509_sctimpl_1045 {
() => {
// Module: crate::x509::sct
// Provides: {"impl_1045"}
// Dependencies: {}
impl SignatureAlgorithm { fn to_attr (& self) -> & 'static str { match self { SignatureAlgorithm :: Rsa => "RSA" , SignatureAlgorithm :: Dsa => "DSA" , SignatureAlgorithm :: Ecdsa => "ECDSA" , } } }
};
}
