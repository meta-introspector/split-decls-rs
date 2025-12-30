// Generated macro for impl_3657 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3657 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3657"}
// Dependencies: {}
impl From < Vec < u8 > > for SqliteBindValue < '_ > { fn from (b : Vec < u8 >) -> Self { Self { inner : InternalSqliteBindValue :: Binary (b . into_boxed_slice ()) , } } }
};
}
