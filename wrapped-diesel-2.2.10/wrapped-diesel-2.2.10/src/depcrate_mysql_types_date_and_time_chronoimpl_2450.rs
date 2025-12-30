// Generated macro for impl_2450 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2450 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2450"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl ToSql < Datetime , Mysql > for NaiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { < NaiveDateTime as ToSql < Timestamp , Mysql > > :: to_sql (self , out) } }
};
}
