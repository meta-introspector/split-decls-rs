// Generated macro for impl_302 (impl)
macro_rules! Depcrate_error_utilimpl_302 {
() => {
// Module: crate::error::util
// Provides: {"impl_302"}
// Dependencies: {}
impl < T : fmt :: Display > fmt :: Display for Quoted < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}{}{}" , self . open , self . body , self . close) } }
};
}
