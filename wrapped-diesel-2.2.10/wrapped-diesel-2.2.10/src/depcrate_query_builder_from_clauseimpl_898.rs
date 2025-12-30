// Generated macro for impl_898 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_898 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_898"}
// Dependencies: {}
impl < DB , F > QueryFragment < DB > for FromClause < F > where F : QuerySource , DB : Backend , F :: FromClause : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { pass . push_sql (" FROM ") ; self . from_clause . walk_ast (pass) } }
};
}
