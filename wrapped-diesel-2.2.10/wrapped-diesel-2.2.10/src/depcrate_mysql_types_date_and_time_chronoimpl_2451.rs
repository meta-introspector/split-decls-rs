// Generated macro for impl_2451 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2451 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2451"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl FromSql < Datetime , Mysql > for NaiveDateTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { < NaiveDateTime as FromSql < Timestamp , Mysql > > :: from_sql (bytes) } }
};
}
