// Generated macro for impl_3953 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3953 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3953"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl ToSql < TimestamptzSqlite , Sqlite > for NaiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (self . format (ENCODE_NAIVE_DATETIME_FORMAT) . to_string ()) ; Ok (IsNull :: No) } }
};
}
