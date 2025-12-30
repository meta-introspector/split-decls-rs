// Generated macro for impl_3711 (impl)
macro_rules! Depcrate_sqlite_connection_owned_rowimpl_3711 {
() => {
// Module: crate::sqlite::connection::owned_row
// Provides: {"impl_3711"}
// Dependencies: {}
impl RowIndex < usize > for OwnedSqliteRow { fn idx (& self , idx : usize) -> Option < usize > { if idx < self . field_count () { Some (idx) } else { None } } }
};
}
