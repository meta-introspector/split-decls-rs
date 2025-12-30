// Generated macro for impl_2550 (impl)
macro_rules! Depcrate_mysql_typesimpl_2550 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2550"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < sql_types :: Integer , Mysql > for i32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_i32 :: < NativeEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
