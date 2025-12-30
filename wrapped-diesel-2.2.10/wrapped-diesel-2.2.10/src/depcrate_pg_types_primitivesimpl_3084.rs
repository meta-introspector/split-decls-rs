// Generated macro for impl_3084 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3084 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3084"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Citext , Pg > for String { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let string = String :: from_utf8 (value . as_bytes () . to_vec ()) ? ; Ok (string) } }
};
}
