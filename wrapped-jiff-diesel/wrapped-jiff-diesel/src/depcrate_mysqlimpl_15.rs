// Generated macro for impl_15 (impl)
macro_rules! Depcrate_mysqlimpl_15 {
() => {
// Module: crate::mysql
// Provides: {"impl_15"}
// Dependencies: {}
impl FromSql < sql_types :: Time , Mysql > for Time { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Time > { let mysql_time = < MysqlTime as FromSql < sql_types :: Time , Mysql > > :: from_sql (bytes) ? ; let nanos = mysql_time . second_part . checked_mul (1_000) . ok_or_else (| | { format ! ("converting second part to nanoseconds overflowed") }) ? ; let time = civil :: Time :: new (mysql_time . hour . try_into () ? , mysql_time . minute . try_into () ? , mysql_time . second . try_into () ? , nanos . try_into () ? ,) ? ; Ok (time . to_diesel ()) } }
};
}
