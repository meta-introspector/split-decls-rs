// Generated macro for impl_3765 (impl)
macro_rules! Depcrate_sqlite_connection_rowimpl_3765 {
() => {
// Module: crate::sqlite::connection::row
// Provides: {"impl_3765"}
// Dependencies: {}
impl < 'idx > RowIndex < & 'idx str > for SqliteRow < '_ , '_ > { fn idx (& self , field_name : & 'idx str) -> Option < usize > { match & mut * self . inner . borrow_mut () { PrivateSqliteRow :: Direct (stmt) => stmt . index_for_column_name (field_name) , PrivateSqliteRow :: Duplicated { column_names , .. } => column_names . iter () . position (| n | n . as_ref () . map (| s | s as & str) == Some (field_name)) , } } }
};
}
