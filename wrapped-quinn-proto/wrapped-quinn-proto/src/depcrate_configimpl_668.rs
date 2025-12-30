// Generated macro for impl_668 (impl)
macro_rules! Depcrate_configimpl_668 {
() => {
// Module: crate::config
// Provides: {"impl_668"}
// Dependencies: {}
impl fmt :: Debug for ClientConfig { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("ClientConfig") . field ("transport" , & self . transport) . field ("version" , & self . version) . finish_non_exhaustive () } }
};
}
