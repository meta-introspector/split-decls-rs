// Generated macro for impl_2541 (impl)
macro_rules! Depcrate_mysql_typesimpl_2541 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2541"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < Unsigned < SmallInt > , Mysql > for u16 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_u16 :: < NativeEndian > (* self) ? ; Ok (IsNull :: No) } }
};
}
