// Generated macro for impl_3002 (impl)
macro_rules! Depcrate_pg_types_jsonimpl_3002 {
() => {
// Module: crate::pg::types::json
// Provides: {"impl_3002"}
// Dependencies: {}
# [cfg (all (feature = "postgres_backend" , feature = "serde_json"))] impl FromSql < sql_types :: Jsonb , Pg > for serde_json :: Value { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let bytes = value . as_bytes () ; if bytes [0] != 1 { return Err ("Unsupported JSONB encoding version" . into ()) ; } serde_json :: from_slice (& bytes [1 ..]) . map_err (| _ | "Invalid Json" . into ()) } }
};
}
