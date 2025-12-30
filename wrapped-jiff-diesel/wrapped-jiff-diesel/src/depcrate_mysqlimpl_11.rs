// Generated macro for impl_11 (impl)
macro_rules! Depcrate_mysqlimpl_11 {
() => {
// Module: crate::mysql
// Provides: {"impl_11"}
// Dependencies: {}
impl FromSql < sql_types :: Timestamp , Mysql > for DateTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < DateTime > { let mysql_time = < MysqlTime as FromSql < sql_types :: Timestamp , Mysql , > > :: from_sql (bytes) ? ; let nanos = mysql_time . second_part . checked_mul (1_000) . ok_or_else (| | { format ! ("converting second part to nanoseconds overflowed") }) ? ; let dt = civil :: DateTime :: new (mysql_time . year . try_into () ? , mysql_time . month . try_into () ? , mysql_time . day . try_into () ? , mysql_time . hour . try_into () ? , mysql_time . minute . try_into () ? , mysql_time . second . try_into () ? , nanos . try_into () ? ,) ? ; Ok (dt . to_diesel ()) } }
};
}
