// Generated macro for InternalSqliteBindValue (enum)
macro_rules! Depcrate_sqlite_connection_bind_collectorInternalSqliteBindValue {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"InternalSqliteBindValue"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum InternalSqliteBindValue < 'a > { BorrowedString (& 'a str) , String (Box < str >) , BorrowedBinary (& 'a [u8]) , Binary (Box < [u8] >) , I32 (i32) , I64 (i64) , F64 (f64) , Null , }
};
}
