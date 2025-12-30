// Generated macro for impl_176 (impl)
macro_rules! Depcrate_errorimpl_176 {
() => {
// Module: crate::error
// Provides: {"impl_176"}
// Dependencies: {}
impl fmt :: Debug for CFError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("CFError") . field ("domain" , & self . domain ()) . field ("code" , & self . code ()) . field ("description" , & self . description ()) . finish () } }
};
}
