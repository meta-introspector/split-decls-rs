// Generated macro for impl_2456 (impl)
macro_rules! Depcrate_mysql_types_date_and_time_chronoimpl_2456 {
() => {
// Module: crate::mysql::types::date_and_time::chrono
// Provides: {"impl_2456"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "mysql_backend"))] impl ToSql < Date , Mysql > for NaiveDate { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { let mysql_time = MysqlTime { year : self . year () . try_into () ? , month : self . month () as libc :: c_uint , day : self . day () as libc :: c_uint , hour : 0 , minute : 0 , second : 0 , second_part : 0 , neg : false , time_type : MysqlTimestampType :: MYSQL_TIMESTAMP_DATE , time_zone_displacement : 0 , } ; < MysqlTime as ToSql < Date , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow ()) } }
};
}
