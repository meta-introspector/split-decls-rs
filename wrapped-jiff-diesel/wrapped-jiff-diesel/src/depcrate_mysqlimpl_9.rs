// Generated macro for impl_9 (impl)
macro_rules! Depcrate_mysqlimpl_9 {
() => {
// Module: crate::mysql
// Provides: {"impl_9"}
// Dependencies: {}
impl FromSql < sql_types :: Datetime , Mysql > for Timestamp { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Timestamp > { let mysql_time = < MysqlTime as FromSql < sql_types :: Datetime , Mysql > > :: from_sql (bytes ,) ? ; let nanos = mysql_time . second_part . checked_mul (1_000) . ok_or_else (| | { format ! ("converting second part to nanoseconds overflowed") }) ? ; let dt = civil :: DateTime :: new (mysql_time . year . try_into () ? , mysql_time . month . try_into () ? , mysql_time . day . try_into () ? , mysql_time . hour . try_into () ? , mysql_time . minute . try_into () ? , mysql_time . second . try_into () ? , nanos . try_into () ? ,) ? ; let offset = tz :: Offset :: from_seconds (mysql_time . time_zone_displacement . try_into () ? ,) ? ; Ok (offset . to_timestamp (dt) ? . to_diesel ()) } }
};
}
