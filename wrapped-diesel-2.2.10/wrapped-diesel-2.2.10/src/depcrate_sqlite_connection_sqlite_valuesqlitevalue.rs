// Generated macro for SqliteValue (struct)
macro_rules! Depcrate_sqlite_connection_sqlite_valueSqliteValue {
() => {
// Module: crate::sqlite::connection::sqlite_value
// Provides: {"SqliteValue"}
// Dependencies: {}
# [doc = " Raw sqlite value as received from the database"] # [doc = ""] # [doc = " Use the `read_*` functions to access the actual"] # [doc = " value or use existing `FromSql` implementations"] # [doc = " to convert this into rust values"] # [allow (missing_debug_implementations , missing_copy_implementations)] pub struct SqliteValue < 'row , 'stmt , 'query > { _row : Option < Ref < 'row , PrivateSqliteRow < 'stmt , 'query > > > , value : NonNull < ffi :: sqlite3_value > , }
};
}
