// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1050 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1050"}
// Dependencies: {}
impl < T , Tab , DB > CanInsertInSingleQuery < DB > for ValuesClause < T , Tab > where DB : Backend , T : CanInsertInSingleQuery < DB > , { fn rows_to_insert (& self) -> Option < usize > { self . values . rows_to_insert () } }
};
}
