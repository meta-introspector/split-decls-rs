// Generated macro for impl_76 (impl)
macro_rules! Depcrate_errorimpl_76 {
() => {
// Module: crate::error
// Provides: {"impl_76"}
// Dependencies: {}
impl < E > Display for ErrorImpl < E > where E : Display , { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Display :: fmt (ErrorImpl :: error (self . erase ()) , formatter) } }
};
}
