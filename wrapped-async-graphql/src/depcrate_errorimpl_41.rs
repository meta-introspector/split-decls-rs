// Generated macro for impl_41 (impl)
macro_rules! Depcrate_errorimpl_41 {
() => {
// Module: crate::error
// Provides: {"impl_41"}
// Dependencies: {}
impl Debug for ServerError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ServerError") . field ("message" , & self . message) . field ("locations" , & self . locations) . field ("path" , & self . path) . field ("extensions" , & self . extensions) . finish () } }
};
}
