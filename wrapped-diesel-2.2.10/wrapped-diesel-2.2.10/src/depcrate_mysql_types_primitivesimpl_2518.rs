// Generated macro for impl_2518 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2518 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2518"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < BigInt , Mysql > for i64 { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { match value . numeric_value () ? { NumericRepresentation :: Tiny (x) => Ok (x . into ()) , NumericRepresentation :: Small (x) => Ok (x . into ()) , NumericRepresentation :: Medium (x) => Ok (x . into ()) , NumericRepresentation :: Big (x) => Ok (x) , NumericRepresentation :: Float (x) => f32_to_i64 (x) , NumericRepresentation :: Double (x) => f64_to_i64 (x) , NumericRepresentation :: Decimal (bytes) => decimal_to_integer (bytes) , } } }
};
}
