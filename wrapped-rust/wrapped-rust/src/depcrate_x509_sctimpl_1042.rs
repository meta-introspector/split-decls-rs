// Generated macro for impl_1042 (impl)
macro_rules! Depcrate_x509_sctimpl_1042 {
() => {
// Module: crate::x509::sct
// Provides: {"impl_1042"}
// Dependencies: {}
impl TryFrom < u8 > for HashAlgorithm { type Error = pyo3 :: PyErr ; fn try_from (value : u8) -> Result < Self , Self :: Error > { Ok (match value { 1 => HashAlgorithm :: Md5 , 2 => HashAlgorithm :: Sha1 , 3 => HashAlgorithm :: Sha224 , 4 => HashAlgorithm :: Sha256 , 5 => HashAlgorithm :: Sha384 , 6 => HashAlgorithm :: Sha512 , _ => { return Err (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("Invalid/unsupported hash algorithm for SCT: {value}"))) } }) } }
};
}
