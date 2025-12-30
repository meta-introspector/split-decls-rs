// Generated macro for impl_16 (impl)
macro_rules! Depcrate_flatimpl_16 {
() => {
// Module: crate::flat
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > fmt :: Debug for FlatStream < 'sval , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatStream") . field ("buffered" , & self . buffered) . field ("state" , & self . state) . finish () } }
};
}
