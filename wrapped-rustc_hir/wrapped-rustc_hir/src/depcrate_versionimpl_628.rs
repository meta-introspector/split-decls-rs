// Generated macro for impl_628 (impl)
macro_rules! Depcrate_versionimpl_628 {
() => {
// Module: crate::version
// Provides: {"impl_628"}
// Dependencies: {}
impl Display for RustcVersion { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "{}.{}.{}" , self . major , self . minor , self . patch) } }
};
}
