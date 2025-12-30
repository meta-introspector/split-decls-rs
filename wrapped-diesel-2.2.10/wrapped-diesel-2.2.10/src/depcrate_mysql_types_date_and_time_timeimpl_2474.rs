// Generated macro for impl_2474 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2474 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2474"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl ToSql < Datetime , Mysql > for OffsetDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { let prim = to_primitive_datetime (* self) ; < PrimitiveDateTime as ToSql < Datetime , Mysql > > :: to_sql (& prim , & mut out . reborrow ()) } }
};
}
