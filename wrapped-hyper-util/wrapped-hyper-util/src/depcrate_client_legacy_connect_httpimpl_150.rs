// Generated macro for impl_150 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_150 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_150"}
// Dependencies: {}
impl fmt :: Debug for ConnectError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut b = f . debug_tuple ("ConnectError") ; b . field (& self . msg) ; if let Some (ref addr) = self . addr { b . field (addr) ; } if let Some (ref cause) = self . cause { b . field (cause) ; } b . finish () } }
};
}
