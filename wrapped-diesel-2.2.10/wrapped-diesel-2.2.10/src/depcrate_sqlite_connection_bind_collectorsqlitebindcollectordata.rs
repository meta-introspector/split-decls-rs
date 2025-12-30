// Generated macro for SqliteBindCollectorData (struct)
macro_rules! Depcrate_sqlite_connection_bind_collectorSqliteBindCollectorData {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"SqliteBindCollectorData"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Sqlite bind collector data that is movable across threads"] pub struct SqliteBindCollectorData { binds : Vec < (OwnedSqliteBindValue , SqliteType) > , }
};
}
