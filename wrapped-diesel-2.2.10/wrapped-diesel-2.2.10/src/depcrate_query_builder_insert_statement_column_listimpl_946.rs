// Generated macro for impl_946 (impl)
macro_rules! Depcrate_query_builder_insert_statement_column_listimpl_946 {
() => {
// Module: crate::query_builder::insert_statement::column_list
// Provides: {"impl_946"}
// Dependencies: {}
impl < C > ColumnList for C where C : Column , { type Table = < C as Column > :: Table ; fn walk_ast < DB : Backend > (& self , mut out : AstPass < '_ , '_ , DB >) -> QueryResult < () > { out . push_identifier (C :: NAME) ? ; Ok (()) } }
};
}
