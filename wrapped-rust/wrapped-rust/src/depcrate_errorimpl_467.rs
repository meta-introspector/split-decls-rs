// Generated macro for impl_467 (impl)
macro_rules! Depcrate_errorimpl_467 {
() => {
// Module: crate::error
// Provides: {"impl_467"}
// Dependencies: {}
impl From < openssl :: error :: ErrorStack > for CryptographyError { fn from (e : openssl :: error :: ErrorStack) -> CryptographyError { CryptographyError :: OpenSSL (e) } }
};
}
