// Generated macro for impl_111 (impl)
macro_rules! Depcrate_errorimpl_111 {
() => {
// Module: crate::error
// Provides: {"impl_111"}
// Dependencies: {}
impl < E > Debug for ErrorImpl < E > where E : Debug , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { ErrorImpl :: debug (self . erase () , formatter) } } }
};
}
