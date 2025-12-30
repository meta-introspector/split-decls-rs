// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1131 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1131"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for SelectClause < T > where DB : Backend , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . 0 . walk_ast (pass) } }
};
}
