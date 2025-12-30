// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl fmt :: Display for Error { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . inner { ErrorInner :: Io (e) => e . fmt (f) , ErrorInner :: Parse (e) => e . fmt (f) , ErrorInner :: Write (e) => e . fmt (f) , ErrorInner :: Modify (e) => e . fmt (f) , } } }
};
}
