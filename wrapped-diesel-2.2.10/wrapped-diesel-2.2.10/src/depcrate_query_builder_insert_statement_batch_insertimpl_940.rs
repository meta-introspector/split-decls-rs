// Generated macro for impl_940 (impl)
macro_rules! Depcrate_query_builder_insert_statement_batch_insertimpl_940 {
() => {
// Module: crate::query_builder::insert_statement::batch_insert
// Provides: {"impl_940"}
// Dependencies: {}
impl < Tab , DB , V , QId , const HAS_STATIC_QUERY_ID : bool > QueryFragment < DB > for BatchInsert < V , Tab , QId , HAS_STATIC_QUERY_ID > where DB : Backend , Self : QueryFragment < DB , DB :: BatchInsertSupport > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: BatchInsertSupport > > :: walk_ast (self , pass) } }
};
}
