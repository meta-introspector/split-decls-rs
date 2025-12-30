// Generated macro for impl_3023 (impl)
macro_rules! Depcrate_pg_types_moneyimpl_3023 {
() => {
// Module: crate::pg::types::money
// Provides: {"impl_3023"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < Money , Pg > for PgMoney { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { FromSql :: < BigInt , Pg > :: from_sql (bytes) . map (PgMoney) } }
};
}
