// Generated macro for impl_159 (impl)
macro_rules! Depcrate_acceptimpl_159 {
() => {
// Module: crate::accept
// Provides: {"impl_159"}
// Dependencies: {}
impl < TlsErr , SvcErr > Error for TlsError < TlsErr , SvcErr > where TlsErr : Error + 'static , SvcErr : Error + 'static , { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { TlsError :: Tls (err) => Some (err) , TlsError :: Service (err) => Some (err) , TlsError :: Timeout => None , } } }
};
}
