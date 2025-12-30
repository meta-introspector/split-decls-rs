// Generated macro for impl_2457 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2457 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2457"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl FromSql < Date , Mysql > for NaiveDate { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let mysql_time = < MysqlTime as FromSql < Date , Mysql > > :: from_sql (bytes) ? ; NaiveDate :: from_ymd_opt (mysql_time . year . try_into () ? , mysql_time . month , mysql_time . day ,) . ok_or_else (| | format ! ("Unable to convert {mysql_time:?} to chrono") . into ()) } }
};
}
