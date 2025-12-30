// Generated macro for impl_3660 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3660 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3660"}
// Dependencies: {}
impl std :: fmt :: Display for InternalSqliteBindValue < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let n = match self { InternalSqliteBindValue :: BorrowedString (_) | InternalSqliteBindValue :: String (_) => { "Text" } InternalSqliteBindValue :: BorrowedBinary (_) | InternalSqliteBindValue :: Binary (_) => { "Binary" } InternalSqliteBindValue :: I32 (_) | InternalSqliteBindValue :: I64 (_) => "Integer" , InternalSqliteBindValue :: F64 (_) => "Float" , InternalSqliteBindValue :: Null => "Null" , } ; f . write_str (n) } }
};
}
