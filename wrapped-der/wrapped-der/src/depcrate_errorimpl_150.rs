// Generated macro for impl_150 (impl)
macro_rules! Depcrate_errorimpl_150 {
() => {
// Module: crate::error
// Provides: {"impl_150"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . kind) ? ; if let Some (pos) = self . position { write ! (f , " at DER byte {pos}") ? ; } Ok (()) } }
};
}
