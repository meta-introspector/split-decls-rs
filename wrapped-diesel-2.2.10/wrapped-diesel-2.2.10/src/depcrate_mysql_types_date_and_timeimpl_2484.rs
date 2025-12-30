// Generated macro for impl_2484 (impl)
macro_rules! Depcrate_mysql_types_date_and_timeimpl_2484 {
() => {
// Module: crate::mysql::types::date_and_time
// Provides: {"impl_2484"}
// Dependencies: {}
impl MysqlTime { # [doc = " Construct a new instance of [MysqlTime]"] # [allow (clippy :: too_many_arguments)] pub fn new (year : libc :: c_uint , month : libc :: c_uint , day : libc :: c_uint , hour : libc :: c_uint , minute : libc :: c_uint , second : libc :: c_uint , second_part : libc :: c_ulong , neg : bool , time_type : MysqlTimestampType , time_zone_displacement : libc :: c_int ,) -> Self { Self { year , month , day , hour , minute , second , second_part , neg , time_type , time_zone_displacement , } } }
};
}
