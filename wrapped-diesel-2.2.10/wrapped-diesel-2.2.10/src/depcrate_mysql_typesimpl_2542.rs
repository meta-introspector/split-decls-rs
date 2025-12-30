// Generated macro for impl_2542 (impl)
macro_rules! Depcrate_mysql_typesimpl_2542 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2542"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < Unsigned < SmallInt > , Mysql > for u16 { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss , clippy :: cast_possible_truncation)] fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let signed : i32 = FromSql :: < Integer , Mysql > :: from_sql (bytes) ? ; Ok (signed as u16) } }
};
}
