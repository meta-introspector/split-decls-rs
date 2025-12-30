// Generated macro for StatementCache (struct)
macro_rules! Depcrate_connection_statement_cacheStatementCache {
() => {
// Module: crate::connection::statement_cache
// Provides: {"StatementCache"}
// Dependencies: {}
# [doc = " A prepared statement cache"] # [allow (missing_debug_implementations , unreachable_pub)] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub struct StatementCache < DB : Backend , Statement > { pub (crate) cache : HashMap < StatementCacheKey < DB > , Statement > , }
};
}
