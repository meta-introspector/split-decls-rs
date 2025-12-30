// Generated macro for impl_3143 (impl)
macro_rules! Depcrate_pg_types_uuidimpl_3143 {
() => {
// Module: crate::pg::types::uuid
// Provides: {"impl_3143"}
// Dependencies: {}
# [cfg (all (feature = "postgres_backend" , feature = "uuid"))] impl FromSql < Uuid , Pg > for uuid :: Uuid { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { uuid :: Uuid :: from_slice (value . as_bytes ()) . map_err (Into :: into) } }
};
}
