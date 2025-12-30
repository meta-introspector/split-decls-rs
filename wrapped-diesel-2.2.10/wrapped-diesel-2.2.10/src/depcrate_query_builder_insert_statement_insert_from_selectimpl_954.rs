// Generated macro for impl_954 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_from_selectimpl_954 {
() => {
// Module: crate::query_builder::insert_statement::insert_from_select
// Provides: {"impl_954"}
// Dependencies: {}
impl < DB , Select , Columns > CanInsertInSingleQuery < DB > for InsertFromSelect < Select , Columns > where DB : Backend , { fn rows_to_insert (& self) -> Option < usize > { None } }
};
}
