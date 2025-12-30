// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_x509_sctimpl_1046 {
() => {
// Module: crate::x509::sct
// Provides: {"impl_1046"}
// Dependencies: {}
impl TryFrom < u8 > for SignatureAlgorithm { type Error = pyo3 :: PyErr ; fn try_from (value : u8) -> Result < Self , Self :: Error > { Ok (match value { 1 => SignatureAlgorithm :: Rsa , 2 => SignatureAlgorithm :: Dsa , 3 => SignatureAlgorithm :: Ecdsa , _ => { return Err (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("Invalid/unsupported signature algorithm for SCT: {value}"))) } }) } }
};
}
