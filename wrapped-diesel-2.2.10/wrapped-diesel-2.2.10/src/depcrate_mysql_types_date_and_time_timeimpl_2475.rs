// Generated macro for impl_2475 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2475 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2475"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl FromSql < Datetime , Mysql > for OffsetDateTime { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let prim = < PrimitiveDateTime as FromSql < Datetime , Mysql > > :: from_sql (bytes) ? ; Ok (prim . assume_offset (UtcOffset :: UTC)) } }
};
}
