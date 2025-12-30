// Generated macro for impl_2497 (impl)
macro_rules! Depcrate_mysql_types_jsonimpl_2497 {
() => {
// Module: crate::mysql::types::json
// Provides: {"impl_2497"}
// Dependencies: {}
# [cfg (all (feature = "mysql_backend" , feature = "serde_json"))] impl FromSql < sql_types :: Json , Mysql > for serde_json :: Value { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { serde_json :: from_slice (value . as_bytes ()) . map_err (| _ | "Invalid Json" . into ()) } }
};
}
