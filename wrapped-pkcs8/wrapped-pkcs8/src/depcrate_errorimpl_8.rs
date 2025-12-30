// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Asn1 (err) => write ! (f , "PKCS#8 ASN.1 error: {err}") , # [cfg (feature = "pkcs5")] Error :: EncryptedPrivateKey (err) => write ! (f , "{err}") , Error :: KeyMalformed => f . write_str ("PKCS#8 cryptographic key data malformed") , Error :: ParametersMalformed => f . write_str ("PKCS#8 algorithm parameters malformed") , Error :: PublicKey (err) => write ! (f , "public key error: {err}") , } } }
};
}
