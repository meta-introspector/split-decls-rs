// Generated macro for impl_3000 (impl)
macro_rules! Depcrate_pg_types_jsonimpl_3000 {
() => {
// Module: crate::pg::types::json
// Provides: {"impl_3000"}
// Dependencies: {}
# [cfg (all (feature = "postgres_backend" , feature = "serde_json"))] impl FromSql < sql_types :: Json , Pg > for serde_json :: Value { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { serde_json :: from_slice (value . as_bytes ()) . map_err (| _ | "Invalid Json" . into ()) } }
};
}
