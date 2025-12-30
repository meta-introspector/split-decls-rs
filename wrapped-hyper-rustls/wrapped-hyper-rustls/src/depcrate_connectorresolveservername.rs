// Generated macro for ResolveServerName (trait)
macro_rules! Depcrate_connectorResolveServerName {
() => {
// Module: crate::connector
// Provides: {"ResolveServerName"}
// Dependencies: {}
# [doc = " A trait implemented by types that can resolve a [`ServerName`] for a request."] pub trait ResolveServerName { # [doc = " Maps a [`Uri`] into a [`ServerName`]."] fn resolve (& self , uri : & Uri ,) -> Result < ServerName < 'static > , Box < dyn std :: error :: Error + Sync + Send > > ; }
};
}
