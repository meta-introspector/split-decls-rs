// Generated macro for impl_2498 (impl)
macro_rules! Depcrate_mysql_types_jsonimpl_2498 {
() => {
// Module: crate::mysql::types::json
// Provides: {"impl_2498"}
// Dependencies: {}
# [cfg (all (feature = "mysql_backend" , feature = "serde_json"))] impl ToSql < sql_types :: Json , Mysql > for serde_json :: Value { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql >) -> serialize :: Result { serde_json :: to_writer (out , self) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
