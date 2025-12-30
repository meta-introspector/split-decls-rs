// Generated macro for impl_3664 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3664 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3664"}
// Dependencies: {}
impl < 'a > std :: convert :: From < & InternalSqliteBindValue < 'a > > for OwnedSqliteBindValue { fn from (value : & InternalSqliteBindValue < 'a >) -> Self { match value { InternalSqliteBindValue :: String (s) => Self :: String (s . clone ()) , InternalSqliteBindValue :: BorrowedString (s) => { Self :: String (String :: from (* s) . into_boxed_str ()) } InternalSqliteBindValue :: Binary (b) => Self :: Binary (b . clone ()) , InternalSqliteBindValue :: BorrowedBinary (s) => { Self :: Binary (Vec :: from (* s) . into_boxed_slice ()) } InternalSqliteBindValue :: I32 (val) => Self :: I32 (* val) , InternalSqliteBindValue :: I64 (val) => Self :: I64 (* val) , InternalSqliteBindValue :: F64 (val) => Self :: F64 (* val) , InternalSqliteBindValue :: Null => Self :: Null , } } }
};
}
