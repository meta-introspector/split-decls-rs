// Generated macro for CachedStatement (struct)
macro_rules! Depcrate_cacheCachedStatement {
() => {
// Module: crate::cache
// Provides: {"CachedStatement"}
// Dependencies: {}
# [doc = " Cacheable statement."] # [doc = ""] # [doc = " Statement will return automatically to the cache by default."] # [doc = " If you want the statement to be discarded, call"] # [doc = " [`discard()`](CachedStatement::discard) on it."] pub struct CachedStatement < 'conn > { stmt : Option < Statement < 'conn > > , cache : & 'conn StatementCache , }
};
}
