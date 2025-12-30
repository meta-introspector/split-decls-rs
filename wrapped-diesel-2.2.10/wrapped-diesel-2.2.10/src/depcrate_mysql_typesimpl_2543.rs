// Generated macro for impl_2543 (impl)
macro_rules! Depcrate_mysql_typesimpl_2543 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2543"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < Unsigned < Integer > , Mysql > for u32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_u32 :: < NativeEndian > (* self) ? ; Ok (IsNull :: No) } }
};
}
