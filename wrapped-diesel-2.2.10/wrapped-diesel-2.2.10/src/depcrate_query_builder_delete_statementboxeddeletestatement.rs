// Generated macro for BoxedDeleteStatement (type)
macro_rules! Depcrate_query_builder_delete_statementBoxedDeleteStatement {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"BoxedDeleteStatement"}
// Dependencies: {}
# [doc = " A `DELETE` statement with a boxed `WHERE` clause"] pub type BoxedDeleteStatement < 'a , DB , T , Ret = NoReturningClause > = DeleteStatement < T , BoxedWhereClause < 'a , DB > , Ret > ;
};
}
