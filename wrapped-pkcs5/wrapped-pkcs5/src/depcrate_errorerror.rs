// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " Given parameters are invalid for this algorithm"] AlgorithmParametersInvalid { # [doc = " OID for algorithm for which the parameters were invalid"] oid : ObjectIdentifier , } , # [doc = " Decryption Failed"] DecryptFailed , # [doc = " Encryption Failed"] EncryptFailed , # [doc = " Pbes1 support is limited to parsing; encryption/decryption is not supported (won't fix)"] # [cfg (feature = "pbes2")] NoPbes1CryptSupport , # [doc = " Algorithm is not supported"] # [doc = ""] # [doc = " This may be due to a disabled crate feature"] # [doc = " Or the algorithm is not supported at all."] UnsupportedAlgorithm { # [doc = " OID of unsupported algorithm"] oid : ObjectIdentifier , } , }
};
}
