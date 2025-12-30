// Generated macro for impl_69 (impl)
macro_rules! Depcrate_builderimpl_69 {
() => {
// Module: crate::builder
// Provides: {"impl_69"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Asn1 (err) => write ! (f , "ASN.1 error: {err}") , Error :: PublicKey (err) => write ! (f , "public key error: {err}") , Error :: Rng => write ! (f , "rng error") , Error :: Signature (err) => write ! (f , "signature error: {err}") , Error :: Builder (message) => write ! (f , "builder error: {message}") , } } }
};
}
