// Generated macro for impl_112 (impl)
macro_rules! Depcrate_errorimpl_112 {
() => {
// Module: crate::error
// Provides: {"impl_112"}
// Dependencies: {}
impl < E > Display for ErrorImpl < E > where E : Display , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { Display :: fmt (ErrorImpl :: error (self . erase ()) , formatter) } } }
};
}
