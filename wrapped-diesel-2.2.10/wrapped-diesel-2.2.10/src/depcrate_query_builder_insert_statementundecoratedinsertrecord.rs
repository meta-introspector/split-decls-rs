// Generated macro for UndecoratedInsertRecord (trait)
macro_rules! Depcrate_query_builder_insert_statementUndecoratedInsertRecord {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"UndecoratedInsertRecord"}
// Dependencies: {}
# [doc = " Marker trait to indicate that no additional operations have been added"] # [doc = " to a record for insert."] # [doc = ""] # [doc = " This is used to prevent things like"] # [doc = " `.on_conflict_do_nothing().on_conflict_do_nothing()`"] # [doc = " from compiling."] # [cfg_attr (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))] pub trait UndecoratedInsertRecord < Table > { }
};
}
