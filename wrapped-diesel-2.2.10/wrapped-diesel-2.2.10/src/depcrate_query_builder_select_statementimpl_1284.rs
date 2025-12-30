// Generated macro for impl_1284 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1284 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1284"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , DB > QueryFragment < DB > for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where DB : Backend , Self : QueryFragment < DB , DB :: SelectStatementSyntax > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: SelectStatementSyntax > > :: walk_ast (self , pass) } }
};
}
