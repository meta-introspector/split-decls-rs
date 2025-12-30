// Generated macro for impl_1165 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1165 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1165"}
// Dependencies: {}
impl < ST , QS , DB , GB > QueryFragment < DB > for BoxedSelectStatement < '_ , ST , QS , DB , GB > where DB : Backend , Self : QueryFragment < DB , DB :: SelectStatementSyntax > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: SelectStatementSyntax > > :: walk_ast (self , pass) } }
};
}
