// Generated macro for impl_3712 (impl)
macro_rules! Depcrate_sqlite_connection_owned_rowimpl_3712 {
() => {
// Module: crate::sqlite::connection::owned_row
// Provides: {"impl_3712"}
// Dependencies: {}
impl < 'idx > RowIndex < & 'idx str > for OwnedSqliteRow { fn idx (& self , field_name : & 'idx str) -> Option < usize > { self . column_names . iter () . position (| n | n . as_ref () . map (| s | s as & str) == Some (field_name)) } }
};
}
