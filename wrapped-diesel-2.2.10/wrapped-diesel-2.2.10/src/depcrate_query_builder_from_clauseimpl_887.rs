// Generated macro for impl_887 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_887 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_887"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for NoFromClause where Self : QueryFragment < DB , DB :: EmptyFromClauseSyntax > , DB : Backend , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: EmptyFromClauseSyntax > > :: walk_ast (self , pass) } }
};
}
