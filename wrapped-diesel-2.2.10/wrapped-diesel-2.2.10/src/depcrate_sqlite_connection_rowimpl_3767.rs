// Generated macro for impl_3767 (impl)
macro_rules! Depcrate_sqlite_connection_rowimpl_3767 {
() => {
// Module: crate::sqlite::connection::row
// Provides: {"impl_3767"}
// Dependencies: {}
impl < 'stmt > Field < 'stmt , Sqlite > for SqliteField < 'stmt , '_ > { fn field_name (& self) -> Option < & str > { match & * self . row { PrivateSqliteRow :: Direct (stmt) => stmt . field_name (self . col_idx . try_into () . expect ("Diesel expects to run at least on a 32 bit platform") ,) , PrivateSqliteRow :: Duplicated { column_names , .. } => column_names . get (self . col_idx) . and_then (| t | t . as_ref () . map (| n | n as & str)) , } } fn is_null (& self) -> bool { self . value () . is_none () } fn value (& self) -> Option < < Sqlite as Backend > :: RawValue < '_ > > { SqliteValue :: new (Ref :: clone (& self . row) , self . col_idx) } }
};
}
