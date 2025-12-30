// Generated macro for BatchInsert (struct)
macro_rules! Depcrate_query_builder_insert_statement_batch_insertBatchInsert {
() => {
// Module: crate::query_builder::insert_statement::batch_insert
// Provides: {"BatchInsert"}
// Dependencies: {}
# [doc = " This type represents a batch insert clause, which allows"] # [doc = " to insert multiple rows at once."] # [doc = ""] # [doc = " Custom backends can specialize the [`QueryFragment`]"] # [doc = " implementation via [`SqlDialect::BatchInsertSupport`]"] # [doc = " or provide fully custom [`ExecuteDsl`](crate::query_dsl::methods::ExecuteDsl)"] # [doc = " and [`LoadQuery`](crate::query_dsl::methods::LoadQuery) implementations"] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] # [derive (Debug)] pub struct BatchInsert < V , Tab , QId , const STABLE_QUERY_ID : bool > { # [doc = " List of values that should be inserted"] pub values : V , _marker : PhantomData < (QId , Tab) > , }
};
}
