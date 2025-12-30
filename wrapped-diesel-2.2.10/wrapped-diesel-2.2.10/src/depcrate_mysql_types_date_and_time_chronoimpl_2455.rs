// Generated macro for impl_2455 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2455 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2455"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl FromSql < Time , Mysql > for NaiveTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let mysql_time = < MysqlTime as FromSql < Time , Mysql > > :: from_sql (bytes) ? ; NaiveTime :: from_hms_opt (mysql_time . hour , mysql_time . minute , mysql_time . second) . ok_or_else (| | format ! ("Unable to convert {mysql_time:?} to chrono") . into ()) } }
};
}
