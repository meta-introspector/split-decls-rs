// Generated macro for impl_3661 (impl)
macro_rules! Depcrate_sqlite_connection_bind_collectorimpl_3661 {
() => {
// Module: crate::sqlite::connection::bind_collector
// Provides: {"impl_3661"}
// Dependencies: {}
impl InternalSqliteBindValue < '_ > { # [allow (unsafe_code)] pub (in crate :: sqlite) fn result_of (self , ctx : & mut libsqlite3_sys :: sqlite3_context ,) -> Result < () , std :: num :: TryFromIntError > { use libsqlite3_sys as ffi ; use std :: os :: raw as libc ; unsafe { match self { InternalSqliteBindValue :: BorrowedString (s) => ffi :: sqlite3_result_text (ctx , s . as_ptr () as * const libc :: c_char , s . len () . try_into () ? , ffi :: SQLITE_TRANSIENT () ,) , InternalSqliteBindValue :: String (s) => ffi :: sqlite3_result_text (ctx , s . as_ptr () as * const libc :: c_char , s . len () . try_into () ? , ffi :: SQLITE_TRANSIENT () ,) , InternalSqliteBindValue :: Binary (b) => ffi :: sqlite3_result_blob (ctx , b . as_ptr () as * const libc :: c_void , b . len () . try_into () ? , ffi :: SQLITE_TRANSIENT () ,) , InternalSqliteBindValue :: BorrowedBinary (b) => ffi :: sqlite3_result_blob (ctx , b . as_ptr () as * const libc :: c_void , b . len () . try_into () ? , ffi :: SQLITE_TRANSIENT () ,) , InternalSqliteBindValue :: I32 (i) => ffi :: sqlite3_result_int (ctx , i as libc :: c_int) , InternalSqliteBindValue :: I64 (l) => ffi :: sqlite3_result_int64 (ctx , l) , InternalSqliteBindValue :: F64 (d) => { ffi :: sqlite3_result_double (ctx , d as libc :: c_double) } InternalSqliteBindValue :: Null => ffi :: sqlite3_result_null (ctx) , } } Ok (()) } }
};
}
