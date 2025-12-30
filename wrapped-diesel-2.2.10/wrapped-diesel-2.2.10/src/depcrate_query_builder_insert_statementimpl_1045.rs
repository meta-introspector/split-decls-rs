// Generated macro for impl_1045 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1045 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1045"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for DefaultValues where DB : Backend , Self : QueryFragment < DB , DB :: DefaultValueClauseForInsert > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: DefaultValueClauseForInsert > > :: walk_ast (self , pass) } }
};
}
