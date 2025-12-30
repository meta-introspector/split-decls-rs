// Generated macro for impl_3988 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3988 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3988"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl ToSql < TimestamptzSqlite , Sqlite > for OffsetDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { let dt_utc = self . to_offset (UtcOffset :: UTC) ; let format = if self . nanosecond () == 0 { ENCODE_DATETIME_FORMAT_WHOLE_SECOND } else { ENCODE_DATETIME_FORMAT_SUBSECOND } ; out . set_value (dt_utc . format (format) . map_err (| err | err . to_string ()) ?) ; Ok (IsNull :: No) } }
};
}
