// Generated macro for impl_2471 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2471 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2471"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl FromSql < Datetime , Mysql > for PrimitiveDateTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let mysql_time = < MysqlTime as FromSql < Timestamp , Mysql > > :: from_sql (bytes) ? ; to_datetime (mysql_time) . map (to_primitive_datetime) . map_err (| err | format ! ("Cannot parse this date: {mysql_time:?}: {err}") . into ()) } }
};
}
