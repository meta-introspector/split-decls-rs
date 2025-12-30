// Generated macro for StatementCacheKey (enum)
macro_rules! Depcrate_connection_statement_cacheStatementCacheKey {
() => {
// Module: crate::connection::statement_cache
// Provides: {"StatementCacheKey"}
// Dependencies: {}
# [doc = " The lookup key used by [`StatementCache`] internally"] # [doc = ""] # [doc = " This can contain either a at compile time known type id"] # [doc = " (representing a statically known query) or a at runtime"] # [doc = " calculated query string + parameter types (for queries"] # [doc = " that may change depending on their parameters)"] # [allow (missing_debug_implementations , unreachable_pub)] # [derive (Hash , PartialEq , Eq)] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub enum StatementCacheKey < DB : Backend > { # [doc = " Represents a at compile time known query"] # [doc = ""] # [doc = " Calculated via [`QueryId::QueryId`]"] Type (TypeId) , # [doc = " Represents a dynamically constructed query"] # [doc = ""] # [doc = " This variant is used if [`QueryId::HAS_STATIC_QUERY_ID`]"] # [doc = " is `false` and [`AstPass::unsafe_to_cache_prepared`] is not"] # [doc = " called for a given query."] Sql { # [doc = " contains the sql query string"] sql : String , # [doc = " contains the types of any bind parameter passed to the query"] bind_types : Vec < DB :: TypeMetadata > , } , }
};
}
