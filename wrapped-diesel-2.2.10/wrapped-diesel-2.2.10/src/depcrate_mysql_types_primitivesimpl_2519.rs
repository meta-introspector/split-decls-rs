// Generated macro for impl_2519 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2519 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2519"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < Float , Mysql > for f32 { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { match value . numeric_value () ? { NumericRepresentation :: Tiny (x) => Ok (x . into ()) , NumericRepresentation :: Small (x) => Ok (x . into ()) , NumericRepresentation :: Medium (x) => Ok (x as Self) , NumericRepresentation :: Big (x) => Ok (x as Self) , NumericRepresentation :: Float (x) => Ok (x) , # [allow (clippy :: cast_possible_truncation)] NumericRepresentation :: Double (x) => Ok (x as Self) , NumericRepresentation :: Decimal (bytes) => Ok (str :: from_utf8 (bytes) ? . parse () ?) , } } }
};
}
