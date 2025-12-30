// Generated macro for SqliteBindValue (struct)
macro_rules! Depcrate_sqlite_connection_bind_collectorSqliteBindValue {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"SqliteBindValue"}
// Dependencies: {}
# [doc = " This type represents a value bound to"] # [doc = " a sqlite prepared statement"] # [doc = ""] # [doc = " It can be constructed via the various `From<T>` implementations"] # [derive (Debug)] pub struct SqliteBindValue < 'a > { pub (in crate :: sqlite) inner : InternalSqliteBindValue < 'a > , }
};
}
