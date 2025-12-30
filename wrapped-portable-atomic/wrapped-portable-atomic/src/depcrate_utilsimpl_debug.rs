// Generated macro for impl_debug (macro)
macro_rules! Depcrate_utilsimpl_debug {
() => {
// Module: crate::utils
// Provides: {"impl_debug"}
// Dependencies: {}
macro_rules ! impl_debug { ($ atomic_type : ident) => { impl fmt :: Debug for $ atomic_type { # [inline] fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . load (Ordering :: Relaxed) , f) } } } ; }
};
}
