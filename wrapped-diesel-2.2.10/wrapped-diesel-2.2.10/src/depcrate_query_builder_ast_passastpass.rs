// Generated macro for AstPass (struct)
macro_rules! Depcrate_query_builder_ast_passAstPass {
() => {
// Module: crate::query_builder::ast_pass
// Provides: {"AstPass"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [doc = " The primary type used when walking a Diesel AST during query execution."] # [doc = ""] # [doc = " Executing a query is generally done in multiple passes. This list includes,"] # [doc = " but is not limited to:"] # [doc = ""] # [doc = " - Generating the SQL"] # [doc = " - Collecting and serializing bound values (sent separately from the SQL)"] # [doc = " - Determining if a query is safe to store in the prepared statement cache"] # [doc = ""] # [doc = " When adding a new type that is used in a Diesel AST, you don't need to care"] # [doc = " about which specific passes are being performed, nor is there any way for"] # [doc = " you to find out what the current pass is. You should simply call the"] # [doc = " relevant methods and trust that they will be a no-op if they're not relevant"] # [doc = " to the current pass."] pub struct AstPass < 'a , 'b , DB > where DB : Backend , DB :: QueryBuilder : 'a , DB :: MetadataLookup : 'a , 'b : 'a , { internals : AstPassInternals < 'a , 'b , DB > , backend : & 'b DB , }
};
}
