// Generated macro for impl_3984 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3984 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3984"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl ToSql < Timestamp , Sqlite > for PrimitiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { let format = if self . nanosecond () == 0 { ENCODE_PRIMITIVE_DATETIME_FORMAT_WHOLE_SECOND } else { ENCODE_PRIMITIVE_DATETIME_FORMAT_SUBSECOND } ; out . set_value (self . format (format) . map_err (| err | err . to_string ()) ?) ; Ok (IsNull :: No) } }
};
}
