// Generated macro for impl_42 (impl)
macro_rules! Depcrate_errorimpl_42 {
() => {
// Module: crate::error
// Provides: {"impl_42"}
// Dependencies: {}
impl < R : RuleType > fmt :: Display for Error < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . format ()) } }
};
}
