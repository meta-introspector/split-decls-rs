// Generated macro for impl_37 (impl)
macro_rules! Depcrate_pkcs8impl_37 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (feature = "pem")] impl fmt :: Display for PublicKeyBytes { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& self . to_public_key_pem (Default :: default ()) . expect ("PEM serialization error") ,) } }
};
}
