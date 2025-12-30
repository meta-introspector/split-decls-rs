// Generated macro for impl_197 (impl)
macro_rules! Depcrate_lengthimpl_197 {
() => {
// Module: crate::length
// Provides: {"impl_197"}
// Dependencies: {}
impl fmt :: Debug for Length { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "ber")] if self . indefinite { return write ! (f , "Length([indefinite])") ; } f . debug_tuple ("Length") . field (& self . inner) . finish () } }
};
}
