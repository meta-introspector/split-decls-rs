// Generated macro for impl_2532 (impl)
macro_rules! Depcrate_mysql_typesimpl_2532 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2532"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl ToSql < TinyInt , Mysql > for i8 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { out . write_i8 (* self) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
