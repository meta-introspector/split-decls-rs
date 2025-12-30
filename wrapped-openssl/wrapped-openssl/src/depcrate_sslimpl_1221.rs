// Generated macro for impl_1221 (impl)
macro_rules! Depcrate_sslimpl_1221 {
() => {
// Module: crate::ssl
// Provides: {"impl_1221"}
// Dependencies: {}
impl fmt :: Debug for SslRef { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Ssl") . field ("state" , & self . state_string_long ()) . field ("verify_result" , & self . verify_result ()) . finish () } }
};
}
