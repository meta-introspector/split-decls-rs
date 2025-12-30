// Generated macro for impl_2479 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2479 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2479"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl FromSql < Time , Mysql > for NaiveTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let mysql_time = < MysqlTime as FromSql < Time , Mysql > > :: from_sql (bytes) ? ; to_time (mysql_time) . map_err (| err | format ! ("Unable to convert {mysql_time:?} to time: {err}") . into ()) } }
};
}
