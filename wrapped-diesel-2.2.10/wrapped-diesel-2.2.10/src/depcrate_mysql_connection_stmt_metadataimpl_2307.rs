// Generated macro for impl_2307 (impl)
macro_rules! Depcrate_mysql_connection_stmt_metadataimpl_2307 {
() => {
// Module: crate::mysql::connection::stmt::metadata
// Provides: {"impl_2307"}
// Dependencies: {}
impl StatementMetadata { pub (in crate :: mysql :: connection) fn new (result : NonNull < ffi :: MYSQL_RES >) -> Self { StatementMetadata { result } } pub (in crate :: mysql :: connection) fn fields (& '_ self) -> & '_ [MysqlFieldMetadata < '_ >] { unsafe { let num_fields = ffi :: mysql_num_fields (self . result . as_ptr ()) ; let field_ptr = ffi :: mysql_fetch_fields (self . result . as_ptr ()) ; if field_ptr . is_null () { & [] } else { slice :: from_raw_parts (field_ptr as _ , num_fields as usize) } } } }
};
}
