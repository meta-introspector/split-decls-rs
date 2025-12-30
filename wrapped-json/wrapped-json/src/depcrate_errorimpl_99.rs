// Generated macro for impl_99 (impl)
macro_rules! Depcrate_errorimpl_99 {
() => {
// Module: crate::error
// Provides: {"impl_99"}
// Dependencies: {}
impl Display for ErrorImpl { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . line == 0 { Display :: fmt (& self . code , f) } else { write ! (f , "{} at line {} column {}" , self . code , self . line , self . column) } } }
};
}
