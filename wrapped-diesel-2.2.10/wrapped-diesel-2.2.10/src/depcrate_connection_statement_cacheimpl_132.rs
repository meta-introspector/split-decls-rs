// Generated macro for impl_132 (impl)
macro_rules! Depcrate_connection_statement_cacheimpl_132 {
() => {
// Module: crate::connection::statement_cache
// Provides: {"impl_132"}
// Dependencies: {}
impl < T > DerefMut for MaybeCached < '_ , T > { fn deref_mut (& mut self) -> & mut Self :: Target { match * self { MaybeCached :: CannotCache (ref mut x) => x , MaybeCached :: Cached (ref mut x) => x , } } }
};
}
