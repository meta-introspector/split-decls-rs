// Generated macro for impl_956 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_from_selectimpl_956 {
() => {
// Module: crate::query_builder::insert_statement::insert_from_select
// Provides: {"impl_956"}
// Dependencies: {}
impl < Select , Columns > UndecoratedInsertRecord < Columns :: Table > for InsertFromSelect < Select , Columns > where Columns : ColumnList + Expression , Select : Query < SqlType = Columns :: SqlType > , { }
};
}
