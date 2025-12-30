// Generated macro for impl_13 (impl)
macro_rules! Depcrate_mysqlimpl_13 {
() => {
// Module: crate::mysql
// Provides: {"impl_13"}
// Dependencies: {}
impl FromSql < sql_types :: Date , Mysql > for Date { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Date > { let mysql_time = < MysqlTime as FromSql < sql_types :: Date , Mysql > > :: from_sql (bytes) ? ; let date = civil :: Date :: new (mysql_time . year . try_into () ? , mysql_time . month . try_into () ? , mysql_time . day . try_into () ? ,) ? ; Ok (date . to_diesel ()) } }
};
}
