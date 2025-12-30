// Generated macro for impl_58 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_58 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_58"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_tuple ("hyper_util::client::legacy::Error") ; f . field (& self . kind) ; if let Some (ref cause) = self . source { f . field (cause) ; } f . finish () } }
};
}
