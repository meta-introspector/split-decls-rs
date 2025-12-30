// Generated macro for impl_2481 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2481 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2481"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl FromSql < Date , Mysql > for NaiveDate { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let mysql_time = < MysqlTime as FromSql < Date , Mysql > > :: from_sql (bytes) ? ; to_datetime (mysql_time) . map_err (| err | format ! ("Unable to convert {mysql_time:?} to time: {err}") . into ()) . and_then (| dt | { let prim = to_primitive_datetime (dt) ; if prim . time () == NaiveTime :: MIDNIGHT { Ok (prim . date ()) } else { Err (format ! ("Unable to convert {prim:?} to date: non-0 time part") . into ()) } }) } }
};
}
