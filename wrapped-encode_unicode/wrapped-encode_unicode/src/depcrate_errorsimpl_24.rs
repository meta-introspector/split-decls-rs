// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorsimpl_24 {
() => {
// Module: crate::errors
// Provides: {"impl_24"}
// Dependencies: {}
impl Display for Utf8Error { fn fmt (& self , fmtr : & mut Formatter) -> fmt :: Result { fmtr . write_str (utf8_error_description (self . kind)) } }
};
}
