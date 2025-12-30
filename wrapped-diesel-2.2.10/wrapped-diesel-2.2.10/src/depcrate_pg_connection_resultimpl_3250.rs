// Generated macro for impl_3250 (impl)
macro_rules! Depcrate_pg_connection_resultimpl_3250 {
() => {
// Module: crate::pg::connection::result
// Provides: {"impl_3250"}
// Dependencies: {}
impl DatabaseErrorInformation for PgErrorInformation { fn message (& self) -> & str { get_result_field (self . 0 . as_ptr () , ResultField :: MessagePrimary) . unwrap_or_else (| | self . 0 . error_message ()) } fn details (& self) -> Option < & str > { get_result_field (self . 0 . as_ptr () , ResultField :: MessageDetail) } fn hint (& self) -> Option < & str > { get_result_field (self . 0 . as_ptr () , ResultField :: MessageHint) } fn table_name (& self) -> Option < & str > { get_result_field (self . 0 . as_ptr () , ResultField :: TableName) } fn column_name (& self) -> Option < & str > { get_result_field (self . 0 . as_ptr () , ResultField :: ColumnName) } fn constraint_name (& self) -> Option < & str > { get_result_field (self . 0 . as_ptr () , ResultField :: ConstraintName) } fn statement_position (& self) -> Option < i32 > { let str_pos = get_result_field (self . 0 . as_ptr () , ResultField :: StatementPosition) ? ; str_pos . parse :: < i32 > () . ok () } }
};
}
