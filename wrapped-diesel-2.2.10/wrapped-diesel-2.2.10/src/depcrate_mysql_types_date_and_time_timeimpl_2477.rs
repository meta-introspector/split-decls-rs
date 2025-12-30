// Generated macro for impl_2477 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2477 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2477"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl FromSql < Timestamp , Mysql > for OffsetDateTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { < OffsetDateTime as FromSql < Datetime , Mysql > > :: from_sql (bytes) } }
};
}
