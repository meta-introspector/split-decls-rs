// Generated macro for BoxedUpdateStatement (type)
macro_rules! Depcrate_query_builder_update_statementBoxedUpdateStatement {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"BoxedUpdateStatement"}
// Dependencies: {}
# [doc = " An `UPDATE` statement with a boxed `WHERE` clause."] pub type BoxedUpdateStatement < 'a , DB , T , V = SetNotCalled , Ret = NoReturningClause > = UpdateStatement < T , BoxedWhereClause < 'a , DB > , V , Ret > ;
};
}
