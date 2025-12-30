// Generated macro for impl_3764 (impl)
macro_rules! Depcrate_sqlite_connection_rowimpl_3764 {
() => {
// Module: crate::sqlite::connection::row
// Provides: {"impl_3764"}
// Dependencies: {}
impl RowIndex < usize > for SqliteRow < '_ , '_ > { fn idx (& self , idx : usize) -> Option < usize > { if idx < self . field_count { Some (idx) } else { None } } }
};
}
