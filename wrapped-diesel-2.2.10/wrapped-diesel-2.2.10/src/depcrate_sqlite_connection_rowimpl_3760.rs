// Generated macro for impl_3760 (impl)
macro_rules! Depcrate_sqlite_connection_rowimpl_3760 {
() => {
// Module: crate::sqlite::connection::row
// Provides: {"impl_3760"}
// Dependencies: {}
impl < 'stmt > IntoOwnedRow < 'stmt , Sqlite > for SqliteRow < 'stmt , '_ > { type OwnedRow = OwnedSqliteRow ; type Cache = Option < Arc < [Option < String >] > > ; fn into_owned (self , column_name_cache : & mut Self :: Cache) -> Self :: OwnedRow { self . inner . borrow () . moveable (column_name_cache) } }
};
}
