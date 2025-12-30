// Generated macro for impl_165 (impl)
macro_rules! Depcrate_bytesimpl_165 {
() => {
// Module: crate::bytes
// Provides: {"impl_165"}
// Dependencies: {}
impl fmt :: Debug for Vtable { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Vtable") . field ("clone" , & (self . clone as * const ())) . field ("drop" , & (self . drop as * const ())) . finish () } }
};
}
