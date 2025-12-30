// Generated macro for impl_3085 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3085 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3085"}
// Dependencies: {}
# [doc = " The returned pointer is *only* valid for the lifetime to the argument of"] # [doc = " `from_sql`. This impl is intended for uses where you want to write a new"] # [doc = " impl in terms of `Vec<u8>`, but don't want to allocate. We have to return a"] # [doc = " raw pointer instead of a reference with a lifetime due to the structure of"] # [doc = " `FromSql`"] # [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Binary , Pg > for * const [u8] { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { Ok (value . as_bytes () as * const _) } }
};
}
