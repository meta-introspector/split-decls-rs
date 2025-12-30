// Generated macro for impl_47 (impl)
macro_rules! Depcrate_diagnosticimpl_47 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_47"}
// Dependencies: {}
impl fmt :: Display for Diagnostic { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (ref rendered) = self . rendered { f . write_str (rendered) ? ; } else { f . write_str ("cargo didn't render this message") ? ; } Ok (()) } }
};
}
