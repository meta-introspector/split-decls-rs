// Generated macro for impl_3714 (impl)
macro_rules! Depcrate_sqlite_connection_owned_rowimpl_3714 {
() => {
// Module: crate::sqlite::connection::owned_row
// Provides: {"impl_3714"}
// Dependencies: {}
impl < 'row > Field < 'row , Sqlite > for OwnedSqliteField < 'row > { fn field_name (& self) -> Option < & str > { self . row . column_names . get (self . col_idx) . and_then (| o | o . as_ref () . map (| s | s . as_ref ())) } fn is_null (& self) -> bool { self . value () . is_none () } fn value (& self) -> Option < < Sqlite as Backend > :: RawValue < 'row > > { SqliteValue :: from_owned_row (self . row , self . col_idx) } }
};
}
