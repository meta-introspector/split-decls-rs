// Generated macro for impl_608 (impl)
macro_rules! Depcrate_errorimpl_608 {
() => {
// Module: crate::error
// Provides: {"impl_608"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("http::Error") . field (& self . get_ref ()) . finish () } }
};
}
