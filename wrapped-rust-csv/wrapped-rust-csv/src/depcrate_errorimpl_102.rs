// Generated macro for impl_102 (impl)
macro_rules! Depcrate_errorimpl_102 {
() => {
// Module: crate::error
// Provides: {"impl_102"}
// Dependencies: {}
impl fmt :: Display for Utf8Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "invalid utf-8: invalid UTF-8 in field {} near byte index {}" , self . field , self . valid_up_to) } }
};
}
