// Generated macro for impl_169 (impl)
macro_rules! Depcrate_errorimpl_169 {
() => {
// Module: crate::error
// Provides: {"impl_169"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_tuple ("hyper::Error") ; f . field (& self . inner . kind) ; if let Some (ref cause) = self . inner . cause { f . field (cause) ; } f . finish () } }
};
}
