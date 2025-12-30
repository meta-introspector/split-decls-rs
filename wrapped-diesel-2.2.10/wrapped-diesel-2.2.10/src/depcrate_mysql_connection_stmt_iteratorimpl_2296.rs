// Generated macro for impl_2296 (impl)
macro_rules! Depcrate_mysql_connection_stmt_iteratorimpl_2296 {
() => {
// Module: crate::mysql::connection::stmt::iterator
// Provides: {"impl_2296"}
// Dependencies: {}
impl < 'a > RowIndex < & 'a str > for MysqlRow { fn idx (& self , idx : & 'a str) -> Option < usize > { self . metadata . fields () . iter () . enumerate () . find (| (_ , field_meta) | field_meta . field_name () == Some (idx)) . map (| (idx , _) | idx) } }
};
}
