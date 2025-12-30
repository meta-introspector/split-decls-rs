// Generated macro for impl_2523 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2523 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2523"}
// Dependencies: {}
# [doc = " The returned pointer is *only* valid for the lifetime to the argument of"] # [doc = " `from_sql`. This impl is intended for uses where you want to write a new"] # [doc = " impl in terms of `Vec<u8>`, but don't want to allocate. We have to return a"] # [doc = " raw pointer instead of a reference with a lifetime due to the structure of"] # [doc = " `FromSql`"] # [cfg (feature = "mysql_backend")] impl FromSql < Binary , Mysql > for * const [u8] { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { Ok (value . as_bytes () as * const [u8]) } }
};
}
