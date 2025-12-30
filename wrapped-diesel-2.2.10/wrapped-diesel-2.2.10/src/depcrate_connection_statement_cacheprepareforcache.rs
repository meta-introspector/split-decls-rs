// Generated macro for PrepareForCache (enum)
macro_rules! Depcrate_connection_statement_cachePrepareForCache {
() => {
// Module: crate::connection::statement_cache
// Provides: {"PrepareForCache"}
// Dependencies: {}
# [doc = " A helper type that indicates if a certain query"] # [doc = " is cached inside of the prepared statement cache or not"] # [doc = ""] # [doc = " This information can be used by the connection implementation"] # [doc = " to signal this fact to the database while actually"] # [doc = " preparing the statement"] # [derive (Debug , Clone , Copy)] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] # [allow (unreachable_pub)] pub enum PrepareForCache { # [doc = " The statement will be cached"] Yes , # [doc = " The statement won't be cached"] No , }
};
}
