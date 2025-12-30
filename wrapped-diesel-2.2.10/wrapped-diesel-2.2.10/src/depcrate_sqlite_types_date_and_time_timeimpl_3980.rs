// Generated macro for impl_3980 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3980 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3980"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl ToSql < Date , Sqlite > for NaiveDate { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Sqlite >) -> serialize :: Result { out . set_value (self . format (DATE_FORMAT) . map_err (| err | err . to_string ()) ?) ; Ok (IsNull :: No) } }
};
}
