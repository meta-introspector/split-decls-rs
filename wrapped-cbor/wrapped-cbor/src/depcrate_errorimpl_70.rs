// Generated macro for impl_70 (impl)
macro_rules! Depcrate_errorimpl_70 {
() => {
// Module: crate::error
// Provides: {"impl_70"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 . offset == 0 { fmt :: Display :: fmt (& self . 0 . code , f) } else { write ! (f , "{} at offset {}" , self . 0 . code , self . 0 . offset) } } }
};
}
