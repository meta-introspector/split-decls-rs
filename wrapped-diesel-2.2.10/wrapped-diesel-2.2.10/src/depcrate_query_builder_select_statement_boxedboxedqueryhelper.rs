// Generated macro for BoxedQueryHelper (trait)
macro_rules! Depcrate_query_builder_select_statement_boxedBoxedQueryHelper {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"BoxedQueryHelper"}
// Dependencies: {}
# [doc (hidden)] pub trait BoxedQueryHelper < 'a , QS , DB > { fn build_query < 'b , 'c > (& 'b self , out : AstPass < '_ , 'c , DB > , where_clause_handler : impl Fn (& 'b BoxedWhereClause < 'a , DB > , AstPass < '_ , 'c , DB > ,) -> QueryResult < () > ,) -> QueryResult < () > where DB : Backend , QS : QueryFragment < DB > , BoxedLimitOffsetClause < 'a , DB > : QueryFragment < DB > , 'b : 'c ; }
};
}
