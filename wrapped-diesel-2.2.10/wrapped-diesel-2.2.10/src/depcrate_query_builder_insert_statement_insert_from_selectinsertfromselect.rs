// Generated macro for InsertFromSelect (struct)
macro_rules! Depcrate_query_builder_insert_statement_insert_from_selectInsertFromSelect {
() => {
// Module: crate::query_builder::insert_statement::insert_from_select
// Provides: {"InsertFromSelect"}
// Dependencies: {}
# [doc = " Represents `(Columns) SELECT FROM ...` for use in an `INSERT` statement"] # [derive (Debug , Clone , Copy , QueryId)] pub struct InsertFromSelect < Select , Columns > { pub (in crate :: query_builder) query : Select , pub (in crate :: query_builder) columns : Columns , }
};
}
