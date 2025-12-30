// Generated macro for SqliteType (enum)
macro_rules! Depcrate_sqlite_backendSqliteType {
() => {
// Module: crate::sqlite::backend
// Provides: {"SqliteType"}
// Dependencies: {}
# [doc = " Determines how a bind parameter is given to SQLite"] # [doc = ""] # [doc = " Diesel deals with bind parameters after serialization as opaque blobs of"] # [doc = " bytes. However, SQLite instead has several functions where it expects the"] # [doc = " relevant C types."] # [doc = ""] # [doc = " The variants of this struct determine what bytes are expected from"] # [doc = " `ToSql` impls."] # [allow (missing_debug_implementations)] # [derive (Debug , Hash , PartialEq , Eq , Clone , Copy)] pub enum SqliteType { # [doc = " Bind using `sqlite3_bind_blob`"] Binary , # [doc = " Bind using `sqlite3_bind_text`"] Text , # [doc = " `bytes` should contain an `f32`"] Float , # [doc = " `bytes` should contain an `f64`"] Double , # [doc = " `bytes` should contain an `i16`"] SmallInt , # [doc = " `bytes` should contain an `i32`"] Integer , # [doc = " `bytes` should contain an `i64`"] Long , }
};
}
