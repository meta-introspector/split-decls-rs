// Generated macro for impl_86 (impl)
macro_rules! Depcrate_errorimpl_86 {
() => {
// Module: crate::error
// Provides: {"impl_86"}
// Dependencies: {}
impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { ErrorImpl :: debug (self . inner . by_ref () , formatter) } } }
};
}
