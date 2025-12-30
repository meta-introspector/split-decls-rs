// Generated macro for impl_43 (impl)
macro_rules! Depcrate_frameimpl_43 {
() => {
// Module: crate::frame
// Provides: {"impl_43"}
// Dependencies: {}
impl fmt :: Display for DisplayTimestamp < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let args = self . frame . format_args (self . frame . timestamp_format . unwrap () , & self . frame . timestamp_args , None ,) ; f . write_str (& args) } }
};
}
