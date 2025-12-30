// Generated macro for impl_2533 (impl)
macro_rules! Depcrate_mysql_typesimpl_2533 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2533"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < TinyInt , Mysql > for i8 { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { let bytes = value . as_bytes () ; Ok (i8 :: from_be_bytes ([bytes [0]])) } }
};
}
