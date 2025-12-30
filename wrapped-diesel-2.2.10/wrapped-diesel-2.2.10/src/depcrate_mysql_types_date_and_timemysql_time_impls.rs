// Generated macro for mysql_time_impls (macro)
macro_rules! Depcrate_mysql_types_date_and_timemysql_time_impls {
() => {
// Module: crate::mysql::types::date_and_time
// Provides: {"mysql_time_impls"}
// Dependencies: {}
macro_rules ! mysql_time_impls { ($ ty : ty) => { # [cfg (feature = "mysql_backend")] impl ToSql <$ ty , Mysql > for MysqlTime { # [allow (unsafe_code)] fn to_sql <'b > (&'b self , out : & mut Output <'b , '_ , Mysql >) -> serialize :: Result { let bytes = unsafe { let bytes_ptr = self as * const MysqlTime as * const u8 ; slice :: from_raw_parts (bytes_ptr , mem :: size_of ::< MysqlTime > ()) } ; out . write_all (bytes) ?; Ok (IsNull :: No) } } # [cfg (feature = "mysql_backend")] impl FromSql <$ ty , Mysql > for MysqlTime { fn from_sql (value : MysqlValue <'_ >) -> deserialize :: Result < Self > { value . time_value () } } } ; }
};
}
