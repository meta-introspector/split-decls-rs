// Generated macro for impl_131 (impl)
macro_rules! Depcrate_connection_statement_cacheimpl_131 {
() => {
// Module: crate::connection::statement_cache
// Provides: {"impl_131"}
// Dependencies: {}
impl < T > Deref for MaybeCached < '_ , T > { type Target = T ; fn deref (& self) -> & Self :: Target { match * self { MaybeCached :: CannotCache (ref x) => x , MaybeCached :: Cached (ref x) => x , } } }
};
}
