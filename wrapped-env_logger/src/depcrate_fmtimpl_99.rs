// Generated macro for impl_99 (impl)
macro_rules! Depcrate_fmtimpl_99 {
() => {
// Module: crate::fmt
// Provides: {"impl_99"}
// Dependencies: {}
impl fmt :: Debug for Formatter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let buf = self . buf . borrow () ; f . debug_struct ("Formatter") . field ("buf" , & buf) . field ("write_style" , & self . write_style) . finish () } }
};
}
