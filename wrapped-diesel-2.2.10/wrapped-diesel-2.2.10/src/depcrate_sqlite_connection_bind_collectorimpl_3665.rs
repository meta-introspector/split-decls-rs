// Generated macro for impl_3665 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3665 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3665"}
// Dependencies: {}
impl std :: convert :: From < & OwnedSqliteBindValue > for InternalSqliteBindValue < '_ > { fn from (value : & OwnedSqliteBindValue) -> Self { match value { OwnedSqliteBindValue :: String (s) => Self :: String (s . clone ()) , OwnedSqliteBindValue :: Binary (b) => Self :: Binary (b . clone ()) , OwnedSqliteBindValue :: I32 (val) => Self :: I32 (* val) , OwnedSqliteBindValue :: I64 (val) => Self :: I64 (* val) , OwnedSqliteBindValue :: F64 (val) => Self :: F64 (* val) , OwnedSqliteBindValue :: Null => Self :: Null , } } }
};
}
