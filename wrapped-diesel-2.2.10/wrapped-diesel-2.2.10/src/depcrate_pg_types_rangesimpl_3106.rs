// Generated macro for impl_3106 (impl)
macro_rules! Depcrate_pg_types_rangesimpl_3106 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"impl_3106"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < T , ST > Queryable < Range < ST > , Pg > for (Bound < T > , Bound < T >) where T : FromSql < ST , Pg > , { type Row = Self ; fn build (row : Self) -> deserialize :: Result < Self > { Ok (row) } }
};
}
