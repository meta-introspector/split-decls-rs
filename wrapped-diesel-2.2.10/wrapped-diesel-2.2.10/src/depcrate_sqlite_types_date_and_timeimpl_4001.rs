// Generated macro for impl_4001 (impl)
macro_rules! Depcrate_sqlite_types_date_and_timeimpl_4001 {
() => {
// Module: crate::sqlite::types::date_and_time
// Provides: {"impl_4001"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: TimestamptzSqlite , Sqlite > for String { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { < str as ToSql < sql_types :: TimestamptzSqlite , Sqlite > > :: to_sql (self as & str , out) } }
};
}
