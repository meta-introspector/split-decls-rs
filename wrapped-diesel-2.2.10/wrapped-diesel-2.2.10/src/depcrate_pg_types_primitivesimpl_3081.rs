// Generated macro for impl_3081 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3081 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3081"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl Queryable < sql_types :: VarChar , Pg > for * const str { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
