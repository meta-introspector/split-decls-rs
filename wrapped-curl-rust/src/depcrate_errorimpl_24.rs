// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorimpl_24 {
() => {
// Module: crate::error
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Debug for MultiError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MultiError") . field ("description" , & self . description ()) . field ("code" , & self . code) . finish () } }
};
}
