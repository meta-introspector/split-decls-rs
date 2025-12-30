// Generated macro for impl_2295 (impl)
macro_rules! Depcrate_mysql_connection_stmt_iteratorimpl_2295 {
() => {
// Module: crate::mysql::connection::stmt::iterator
// Provides: {"impl_2295"}
// Dependencies: {}
impl RowIndex < usize > for MysqlRow { fn idx (& self , idx : usize) -> Option < usize > { if idx < self . field_count () { Some (idx) } else { None } } }
};
}
