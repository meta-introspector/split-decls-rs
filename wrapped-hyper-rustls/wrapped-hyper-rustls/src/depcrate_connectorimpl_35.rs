// Generated macro for impl_35 (impl)
macro_rules! Depcrate_connectorimpl_35 {
() => {
// Module: crate::connector
// Provides: {"impl_35"}
// Dependencies: {}
impl ResolveServerName for FixedServerNameResolver { fn resolve (& self , _ : & Uri ,) -> Result < ServerName < 'static > , Box < dyn std :: error :: Error + Sync + Send > > { Ok (self . name . clone ()) } }
};
}
