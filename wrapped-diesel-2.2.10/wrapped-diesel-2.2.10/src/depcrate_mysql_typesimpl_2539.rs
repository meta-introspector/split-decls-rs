// Generated macro for impl_2539 (impl)
macro_rules! Depcrate_mysql_typesimpl_2539 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2539"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < Unsigned < TinyInt > , Mysql > for u8 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_u8 (* self) ? ; Ok (IsNull :: No) } }
};
}
