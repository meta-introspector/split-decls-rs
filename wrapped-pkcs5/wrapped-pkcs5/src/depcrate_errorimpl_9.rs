// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: AlgorithmParametersInvalid { oid } => { write ! (f , "PKCS#5 parameters for algorithm {oid} are invalid") } Error :: DecryptFailed => f . write_str ("PKCS#5 decryption failed") , Error :: EncryptFailed => f . write_str ("PKCS#5 encryption failed") , # [cfg (feature = "pbes2")] Error :: NoPbes1CryptSupport => { f . write_str ("PKCS#5 encryption/decryption unsupported for PBES1 (won't fix)") } Error :: UnsupportedAlgorithm { oid } => { write ! (f , "PKCS#5 algorithm {oid} is unsupported") } } } }
};
}
