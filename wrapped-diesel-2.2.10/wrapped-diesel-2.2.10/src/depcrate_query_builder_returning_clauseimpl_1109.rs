// Generated macro for impl_1109 (impl)
macro_rules! Depcrate_query_builder_returning_clauseimpl_1109 {
() => {
// Module: crate::query_builder::returning_clause
// Provides: {"impl_1109"}
// Dependencies: {}
impl < Expr , DB > QueryFragment < DB > for ReturningClause < Expr > where DB : Backend , Self : QueryFragment < DB , DB :: ReturningClause > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: ReturningClause > > :: walk_ast (self , pass) } }
};
}
