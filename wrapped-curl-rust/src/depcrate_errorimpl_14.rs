// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorimpl_14 {
() => {
// Module: crate::error
// Provides: {"impl_14"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Error") . field ("description" , & self . description ()) . field ("code" , & self . code) . field ("extra" , & self . extra) . finish () } }
};
}
