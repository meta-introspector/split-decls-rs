// Generated macro for impl_2520 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2520 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2520"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < Double , Mysql > for f64 { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { match value . numeric_value () ? { NumericRepresentation :: Tiny (x) => Ok (x . into ()) , NumericRepresentation :: Small (x) => Ok (x . into ()) , NumericRepresentation :: Medium (x) => Ok (x . into ()) , NumericRepresentation :: Big (x) => Ok (x as Self) , NumericRepresentation :: Float (x) => Ok (x . into ()) , NumericRepresentation :: Double (x) => Ok (x) , NumericRepresentation :: Decimal (bytes) => Ok (str :: from_utf8 (bytes) ? . parse () ?) , } } }
};
}
