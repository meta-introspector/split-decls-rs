// Generated macro for impl_470 (impl)
macro_rules! Depcrate_errorimpl_470 {
() => {
// Module: crate::error
// Provides: {"impl_470"}
// Dependencies: {}
impl From < cryptography_key_parsing :: KeySerializationError > for CryptographyError { fn from (e : cryptography_key_parsing :: KeySerializationError) -> CryptographyError { match e { cryptography_key_parsing :: KeySerializationError :: PasswordMustBeUtf8 => { CryptographyError :: Py (pyo3 :: exceptions :: PyValueError :: new_err ("password must be valid UTF-8" ,)) } cryptography_key_parsing :: KeySerializationError :: Write (e) => { CryptographyError :: Asn1Write (e) } cryptography_key_parsing :: KeySerializationError :: OpenSSL (e) => { CryptographyError :: OpenSSL (e) } } } }
};
}
