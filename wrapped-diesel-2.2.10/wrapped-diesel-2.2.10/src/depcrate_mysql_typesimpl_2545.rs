// Generated macro for impl_2545 (impl)
macro_rules! Depcrate_mysql_typesimpl_2545 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2545"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < Unsigned < BigInt > , Mysql > for u64 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_u64 :: < NativeEndian > (* self) ? ; Ok (IsNull :: No) } }
};
}
