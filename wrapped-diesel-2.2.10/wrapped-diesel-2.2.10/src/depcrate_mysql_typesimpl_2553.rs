// Generated macro for impl_2553 (impl)
macro_rules! Depcrate_mysql_typesimpl_2553 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2553"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < sql_types :: Float , Mysql > for f32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_f32 :: < NativeEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
