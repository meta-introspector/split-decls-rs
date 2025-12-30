// Generated macro for impl_29 (impl)
macro_rules! Depcrate_errorimpl_29 {
() => {
// Module: crate::error
// Provides: {"impl_29"}
// Dependencies: {}
impl fmt :: Debug for FormError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("FormError") . field ("description" , & self . description ()) . field ("code" , & self . code) . finish () } }
};
}
