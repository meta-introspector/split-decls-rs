// Generated macro for impl_2548 (impl)
macro_rules! Depcrate_mysql_typesimpl_2548 {
() => {
// Module: crate::mysql::types
// Provides: {"impl_2548"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < Bool , Mysql > for bool { fn from_sql (bytes : MysqlValue < '_ >) -> deserialize :: Result < Self > { Ok (bytes . as_bytes () . iter () . any (| x | * x != 0)) } }
};
}
