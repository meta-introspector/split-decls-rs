// Generated macro for impl_2544 (impl)
macro_rules! Depcrate_mysql_typesimpl_2544 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2544"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < Unsigned < Integer > , Mysql > for u32 { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss , clippy :: cast_possible_truncation)] fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let signed : i64 = FromSql :: < BigInt , Mysql > :: from_sql (bytes) ? ; Ok (signed as u32) } }
};
}
