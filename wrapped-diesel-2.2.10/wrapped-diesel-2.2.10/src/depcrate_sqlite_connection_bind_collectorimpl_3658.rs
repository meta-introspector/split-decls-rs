// Generated macro for impl_3658 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3658 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3658"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for SqliteBindValue < 'a > { fn from (b : & 'a [u8]) -> Self { Self { inner : InternalSqliteBindValue :: BorrowedBinary (b) , } } }
};
}
