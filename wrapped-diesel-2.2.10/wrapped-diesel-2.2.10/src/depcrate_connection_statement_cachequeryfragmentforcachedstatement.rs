// Generated macro for QueryFragmentForCachedStatement (trait)
macro_rules! Depcrate_connection_statement_cacheQueryFragmentForCachedStatement {
() => {
// Module: crate::connection::statement_cache
// Provides: {"QueryFragmentForCachedStatement"}
// Dependencies: {}
# [doc = " Implemented for all `QueryFragment`s, dedicated to dynamic dispatch within the context of"] # [doc = " `statement_cache`"] # [doc = ""] # [doc = " We want the generated code to be as small as possible, so for each query passed to"] # [doc = " [`StatementCache::cached_statement`] the generated assembly will just call a non generic"] # [doc = " version with dynamic dispatch pointing to the VTABLE of this minimal trait"] # [doc = ""] # [doc = " This preserves the opportunity for the compiler to entirely optimize the `construct_sql`"] # [doc = " function as a function that simply returns a constant `String`."] # [allow (unreachable_pub)] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub trait QueryFragmentForCachedStatement < DB > { # [doc = " Convert the query fragment into a SQL string for the given backend"] fn construct_sql (& self , backend : & DB) -> QueryResult < String > ; # [doc = " Check whether it's safe to cache the query"] fn is_safe_to_cache_prepared (& self , backend : & DB) -> QueryResult < bool > ; }
};
}
