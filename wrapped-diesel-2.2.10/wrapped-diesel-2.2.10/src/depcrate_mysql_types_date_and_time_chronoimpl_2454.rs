// Generated macro for impl_2454 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2454 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2454"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl ToSql < Time , Mysql > for NaiveTime { fn to_sql < 'b > (& 'b self , out : & mut serialize :: Output < 'b , '_ , Mysql >) -> serialize :: Result { let mysql_time = MysqlTime { hour : self . hour () as libc :: c_uint , minute : self . minute () as libc :: c_uint , second : self . second () as libc :: c_uint , day : 0 , month : 0 , second_part : 0 , year : 0 , neg : false , time_type : MysqlTimestampType :: MYSQL_TIMESTAMP_TIME , time_zone_displacement : 0 , } ; < MysqlTime as ToSql < Time , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow ()) } }
};
}
