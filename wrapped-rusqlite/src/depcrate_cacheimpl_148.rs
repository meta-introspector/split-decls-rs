// Generated macro for impl_148 (impl)
macro_rules! Depcrate_cacheimpl_148 {
() => {
// Module: crate::cache
// Provides: {"impl_148"}
// Dependencies: {}
impl CachedStatement < '_ > { # [inline] fn new < 'conn > (stmt : Statement < 'conn > , cache : & 'conn StatementCache) -> CachedStatement < 'conn > { CachedStatement { stmt : Some (stmt) , cache , } } # [doc = " Discard the statement, preventing it from being returned to its"] # [doc = " [`Connection`]'s collection of cached statements."] # [inline] pub fn discard (mut self) { self . stmt = None ; } }
};
}
