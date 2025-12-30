// Generated macro for lookup_resolver (function)
macro_rules! Depcrate_resolverlookup_resolver {
() => {
// Module: crate::resolver
// Provides: {"lookup_resolver"}
// Dependencies: {}
fn lookup_resolver (db : & dyn DefDatabase , lookup : impl Lookup < Database = dyn DefDatabase , Data = impl AstIdLoc < Container = impl HasResolver > > ,) -> Resolver < '_ > { lookup . lookup (db) . container () . resolver (db) }
};
}
