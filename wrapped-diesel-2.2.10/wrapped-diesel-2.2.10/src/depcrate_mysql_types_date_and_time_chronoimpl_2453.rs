// Generated macro for impl_2453 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2453 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2453"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl FromSql < Timestamp , Mysql > for NaiveDateTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let mysql_time = < MysqlTime as FromSql < Timestamp , Mysql > > :: from_sql (bytes) ? ; let micro = mysql_time . second_part . try_into () ? ; NaiveDate :: from_ymd_opt (mysql_time . year . try_into () ? , mysql_time . month , mysql_time . day ,) . and_then (| v | { v . and_hms_micro_opt (mysql_time . hour , mysql_time . minute , mysql_time . second , micro) }) . ok_or_else (| | format ! ("Cannot parse this date: {mysql_time:?}") . into ()) } }
};
}
