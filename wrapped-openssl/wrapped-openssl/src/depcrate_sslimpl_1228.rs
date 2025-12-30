// Generated macro for impl_1228 (impl)
macro_rules! Depcrate_sslimpl_1228 {
() => {
// Module: crate::ssl
// Provides: {"impl_1228"}
// Dependencies: {}
impl < S > fmt :: Debug for SslStream < S > where S : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("SslStream") . field ("stream" , & self . get_ref ()) . field ("ssl" , & self . ssl ()) . finish () } }
};
}
