// Generated macro for impl_2452 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2452 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2452"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl ToSql < Timestamp , Mysql > for NaiveDateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { let mysql_time = MysqlTime { year : self . year () . try_into () ? , month : self . month () as libc :: c_uint , day : self . day () as libc :: c_uint , hour : self . hour () as libc :: c_uint , minute : self . minute () as libc :: c_uint , second : self . second () as libc :: c_uint , # [allow (deprecated)] second_part : libc :: c_ulong :: from (self . timestamp_subsec_micros ()) , neg : false , time_type : MysqlTimestampType :: MYSQL_TIMESTAMP_DATETIME , time_zone_displacement : 0 , } ; < MysqlTime as ToSql < Timestamp , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow ()) } }
};
}
