// Generated macro for impl_71 (impl)
macro_rules! Depcrate_public_keyimpl_71 {
() => {
// Module: crate::public_key
// Provides: {"impl_71"}
// Dependencies: {}
# [cfg (feature = "pem")] impl Display for PublicKey { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . to_public_key_pem (Default :: default ()) . expect ("PEM encoding error")) } }
};
}
