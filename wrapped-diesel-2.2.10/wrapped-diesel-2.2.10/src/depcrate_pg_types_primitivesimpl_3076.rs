// Generated macro for impl_3076 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3076 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3076"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: CChar , Pg > for u8 { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { Ok (bytes . as_bytes () [0]) } }
};
}
