// Generated macro for impl_3982 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3982 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3982"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl ToSql < Time , Sqlite > for NaiveTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { let format = if self . microsecond () == 0 { ENCODE_TIME_FORMAT_WHOLE_SECOND } else { ENCODE_TIME_FORMAT_SUBSECOND } ; out . set_value (self . format (format) . map_err (| err | err . to_string ()) ?) ; Ok (IsNull :: No) } }
};
}
