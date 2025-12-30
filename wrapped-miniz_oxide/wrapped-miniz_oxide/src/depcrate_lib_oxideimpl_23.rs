// Generated macro for impl_23 (impl)
macro_rules! Depcrate_lib_oxideimpl_23 {
() => {
// Module: crate::lib_oxide
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Debug for InternalState { # [cold] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let name = match & self { InternalState :: Inflate (_) => "Decompressor" , InternalState :: Deflate (_) => "Compressor" , } ; f . write_str (name) } }
};
}
