// Generated macro for impl_204 (impl)
macro_rules! Depcrate_errorimpl_204 {
() => {
// Module: crate::error
// Provides: {"impl_204"}
// Dependencies: {}
impl fmt :: Display for Span { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start == self . end { write ! (f , "{}" , self . start) } else { write ! (f , "{}-{}" , self . start , self . end) } } }
};
}
