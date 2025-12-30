// Generated macro for impl_468 (impl)
macro_rules! Depcrate_errorimpl_468 {
() => {
// Module: crate::error
// Provides: {"impl_468"}
// Dependencies: {}
impl From < pem :: PemError > for CryptographyError { fn from (e : pem :: PemError) -> CryptographyError { CryptographyError :: Py (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("Unable to load PEM file. See https://cryptography.io/en/latest/faq/#why-can-t-i-import-my-pem-file for more details. {e:?}"))) } }
};
}
