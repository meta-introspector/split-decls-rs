// Generated macro for impl_4000 (impl)
macro_rules! Depcrate_sqlite_types_date_and_timeimpl_4000 {
() => {
// Module: crate::sqlite::types::date_and_time
// Provides: {"impl_4000"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl ToSql < sql_types :: TimestamptzSqlite , Sqlite > for str { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { ToSql :: < sql_types :: Text , Sqlite > :: to_sql (self , out) } }
};
}
