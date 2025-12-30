// Generated macro for impl_198 (impl)
macro_rules! Depcrate_errorimpl_198 {
() => {
// Module: crate::error
// Provides: {"impl_198"}
// Dependencies: {}
impl fmt :: Display for SpannedError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}: {}" , self . span , self . code) } }
};
}
