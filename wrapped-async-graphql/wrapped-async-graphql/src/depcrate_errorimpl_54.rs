// Generated macro for impl_54 (impl)
macro_rules! Depcrate_errorimpl_54 {
() => {
// Module: crate::error
// Provides: {"impl_54"}
// Dependencies: {}
impl Debug for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Error") . field ("message" , & self . message) . field ("extensions" , & self . extensions) . finish () } }
};
}
