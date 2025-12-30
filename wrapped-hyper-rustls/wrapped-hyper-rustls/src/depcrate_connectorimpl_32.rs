// Generated macro for impl_32 (impl)
macro_rules! Depcrate_connectorimpl_32 {
() => {
// Module: crate::connector
// Provides: {"impl_32"}
// Dependencies: {}
impl ResolveServerName for DefaultServerNameResolver { fn resolve (& self , uri : & Uri ,) -> Result < ServerName < 'static > , Box < dyn std :: error :: Error + Sync + Send > > { let mut hostname = uri . host () . unwrap_or_default () ; if let Some (trimmed) = hostname . strip_prefix ('[') . and_then (| h | h . strip_suffix (']')) { hostname = trimmed ; } ServerName :: try_from (hostname . to_string ()) . map_err (| e | Box :: new (e) as _) } }
};
}
