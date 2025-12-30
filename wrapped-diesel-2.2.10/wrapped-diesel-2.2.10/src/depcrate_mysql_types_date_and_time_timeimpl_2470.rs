// Generated macro for impl_2470 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_timeimpl_2470 {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"impl_2470"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "mysql_backend"))] impl ToSql < Datetime , Mysql > for PrimitiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { let mysql_time = MysqlTime { year : self . year () . try_into () ? , month : self . month () as libc :: c_uint , day : self . day () as libc :: c_uint , hour : self . hour () as libc :: c_uint , minute : self . minute () as libc :: c_uint , second : self . second () as libc :: c_uint , second_part : libc :: c_ulong :: from (self . microsecond ()) , neg : false , time_type : MysqlTimestampType :: MYSQL_TIMESTAMP_DATETIME , time_zone_displacement : 0 , } ; < MysqlTime as ToSql < Timestamp , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow ()) } }
};
}
