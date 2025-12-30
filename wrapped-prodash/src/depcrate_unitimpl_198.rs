// Generated macro for impl_198 (impl)
macro_rules! Depcrate_unitimpl_198 {
() => {
// Module: crate::unit
// Provides: {"impl_198"}
// Dependencies: {}
impl fmt :: Debug for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Kind :: Label (name) => f . write_fmt (format_args ! ("Unit::Label({:?})" , name)) , Kind :: Dynamic (_) => f . write_fmt (format_args ! ("Unit::Dynamic(..)")) , } } }
};
}
