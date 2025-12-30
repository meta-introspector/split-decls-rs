// Generated macro for impl_75 (impl)
macro_rules! Depcrate_errorimpl_75 {
() => {
// Module: crate::error
// Provides: {"impl_75"}
// Dependencies: {}
impl < E > Debug for ErrorImpl < E > where E : Debug , { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ErrorImpl :: debug (self . erase () , formatter) } }
};
}
