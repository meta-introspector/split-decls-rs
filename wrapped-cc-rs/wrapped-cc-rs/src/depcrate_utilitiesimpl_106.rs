// Generated macro for impl_106 (impl)
macro_rules! Depcrate_utilitiesimpl_106 {
() => {
// Module: crate::utilities
// Provides: {"impl_106"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for OnceLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_tuple ("OnceLock") ; match self . get () { Some (v) => d . field (v) , None => d . field (& format_args ! ("<uninit>")) , } ; d . finish () } }
};
}
