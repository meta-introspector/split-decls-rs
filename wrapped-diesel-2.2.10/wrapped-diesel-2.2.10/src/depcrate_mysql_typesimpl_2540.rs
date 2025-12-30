// Generated macro for impl_2540 (impl)
macro_rules! Depcrate_mysql_typesimpl_2540 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2540"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < Unsigned < TinyInt > , Mysql > for u8 { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss)] fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { let signed : i8 = FromSql :: < TinyInt , Mysql > :: from_sql (bytes) ? ; Ok (signed as u8) } }
};
}
