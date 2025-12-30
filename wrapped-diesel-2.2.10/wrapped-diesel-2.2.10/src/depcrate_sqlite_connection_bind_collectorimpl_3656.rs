// Generated macro for impl_3656 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3656 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3656"}
// Dependencies: {}
impl From < String > for SqliteBindValue < '_ > { fn from (s : String) -> Self { Self { inner : InternalSqliteBindValue :: String (s . into_boxed_str ()) , } } }
};
}
