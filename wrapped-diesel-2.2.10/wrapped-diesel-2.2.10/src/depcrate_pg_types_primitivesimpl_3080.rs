// Generated macro for impl_3080 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3080 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3080"}
// Dependencies: {}
# [doc = " The returned pointer is *only* valid for the lifetime to the argument of"] # [doc = " `from_sql`. This impl is intended for uses where you want to write a new"] # [doc = " impl in terms of `String`, but don't want to allocate. We have to return a"] # [doc = " raw pointer instead of a reference with a lifetime due to the structure of"] # [doc = " `FromSql`"] # [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Text , Pg > for * const str { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { use std :: str ; let string = str :: from_utf8 (value . as_bytes ()) ? ; Ok (string as * const _) } }
};
}
