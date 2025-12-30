// Generated macro for impl_85 (impl)
macro_rules! Depcrate_errorimpl_85 {
() => {
// Module: crate::error
// Provides: {"impl_85"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { ErrorImpl :: display (self . inner . by_ref () , formatter) } } }
};
}
