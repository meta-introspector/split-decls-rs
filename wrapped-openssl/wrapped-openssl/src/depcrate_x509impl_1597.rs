// Generated macro for impl_1597 (impl)
macro_rules! Depcrate_x509impl_1597 {
() => {
// Module: crate::x509
// Provides: {"impl_1597"}
// Dependencies: {}
impl fmt :: Debug for X509VerifyResult { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("X509VerifyResult") . field ("code" , & self . 0) . field ("error" , & self . error_string ()) . finish () } }
};
}
