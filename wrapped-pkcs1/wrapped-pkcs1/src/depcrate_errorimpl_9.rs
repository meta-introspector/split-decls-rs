// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Asn1 (err) => write ! (f , "PKCS#1 ASN.1 error: {err}") , Error :: KeyMalformed => f . write_str ("PKCS#1 cryptographic key data malformed") , Error :: Crypto => f . write_str ("PKCS#1 cryptographic error") , Error :: Version => f . write_str ("PKCS#1 version error") , } } }
};
}
