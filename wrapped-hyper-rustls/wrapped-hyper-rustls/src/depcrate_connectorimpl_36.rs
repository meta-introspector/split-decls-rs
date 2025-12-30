// Generated macro for impl_36 (impl)
macro_rules! Depcrate_connectorimpl_36 {
() => {
// Module: crate::connector
// Provides: {"impl_36"}
// Dependencies: {}
impl < F , E > ResolveServerName for F where F : Fn (& Uri) -> Result < ServerName < 'static > , E > , E : Into < Box < dyn std :: error :: Error + Sync + Send > > , { fn resolve (& self , uri : & Uri ,) -> Result < ServerName < 'static > , Box < dyn std :: error :: Error + Sync + Send > > { self (uri) . map_err (Into :: into) } }
};
}
