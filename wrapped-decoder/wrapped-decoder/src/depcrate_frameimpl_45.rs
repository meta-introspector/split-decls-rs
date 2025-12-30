// Generated macro for impl_45 (impl)
macro_rules! Depcrate_frameimpl_45 {
() => {
// Module: crate::frame
// Provides: {"impl_45"}
// Dependencies: {}
impl fmt :: Display for DisplayMessage < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let args = self . frame . format_args (self . frame . format , & self . frame . args , None) ; f . write_str (& args) } }
};
}
