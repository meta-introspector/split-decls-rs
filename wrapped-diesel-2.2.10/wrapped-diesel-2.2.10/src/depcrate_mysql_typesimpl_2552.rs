// Generated macro for impl_2552 (impl)
macro_rules! Depcrate_mysql_typesimpl_2552 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2552"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < sql_types :: Double , Mysql > for f64 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_f64 :: < NativeEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
