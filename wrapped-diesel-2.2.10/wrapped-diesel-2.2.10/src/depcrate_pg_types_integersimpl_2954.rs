// Generated macro for impl_2954 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2954 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2954"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Oid , Pg > for u32 { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = bytes . as_bytes () ; bytes . read_u32 :: < NetworkEndian > () . map_err (Into :: into) } }
};
}
