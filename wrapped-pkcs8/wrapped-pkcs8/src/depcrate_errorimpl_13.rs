// Generated macro for impl_13 (impl)
macro_rules! Depcrate_errorimpl_13 {
() => {
// Module: crate::error
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "pkcs5")] impl From < pkcs5 :: Error > for Error { fn from (err : pkcs5 :: Error) -> Error { Error :: EncryptedPrivateKey (err) } }
};
}
