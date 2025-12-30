// Generated macro for impl_100 (impl)
macro_rules! Depcrate_errorimpl_100 {
() => {
// Module: crate::error
// Provides: {"impl_100"}
// Dependencies: {}
impl Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Error({:?}, line: {}, column: {})" , self . err . code . to_string () , self . err . line , self . err . column) } }
};
}
