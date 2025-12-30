// Generated macro for impl_3956 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3956 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3956"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl < TZ : TimeZone > ToSql < TimestamptzSqlite , Sqlite > for DateTime < TZ > { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { let dt_utc = self . with_timezone (& Utc) ; out . set_value (dt_utc . format (ENCODE_DATETIME_FORMAT) . to_string ()) ; Ok (IsNull :: No) } }
};
}
