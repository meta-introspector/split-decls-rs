// Generated macro for impl_2521 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2521 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2521"}
// Dependencies: {}
# [doc = " The returned pointer is *only* valid for the lifetime to the argument of"] # [doc = " `from_sql`. This impl is intended for uses where you want to write a new"] # [doc = " impl in terms of `String`, but don't want to allocate. We have to return a"] # [doc = " raw pointer instead of a reference with a lifetime due to the structure of"] # [doc = " `FromSql`"] # [cfg (feature = "mysql_backend")] impl FromSql < Text , Mysql > for * const str { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { let string = str :: from_utf8 (value . as_bytes ()) ? ; Ok (string as * const str) } }
};
}
