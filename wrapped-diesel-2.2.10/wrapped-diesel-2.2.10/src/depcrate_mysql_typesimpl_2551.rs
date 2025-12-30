// Generated macro for impl_2551 (impl)
macro_rules! Depcrate_mysql_typesimpl_2551 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2551"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < sql_types :: BigInt , Mysql > for i64 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_i64 :: < NativeEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
