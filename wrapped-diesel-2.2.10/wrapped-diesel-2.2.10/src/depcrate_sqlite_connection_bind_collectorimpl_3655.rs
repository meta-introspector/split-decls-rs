// Generated macro for impl_3655 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3655 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3655"}
// Dependencies: {}
impl < 'a > From < & 'a str > for SqliteBindValue < 'a > { fn from (s : & 'a str) -> Self { Self { inner : InternalSqliteBindValue :: BorrowedString (s) , } } }
};
}
