// Generated macro for impl_2549 (impl)
macro_rules! Depcrate_mysql_typesimpl_2549 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2549"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < sql_types :: SmallInt , Mysql > for i16 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_i16 :: < NativeEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
