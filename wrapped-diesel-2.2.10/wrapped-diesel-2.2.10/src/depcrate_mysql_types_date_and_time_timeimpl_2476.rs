// Generated macro for impl_2476 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2476 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2476"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl ToSql < Timestamp , Mysql > for OffsetDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { < OffsetDateTime as ToSql < Datetime , Mysql > > :: to_sql (self , out) } }
};
}
