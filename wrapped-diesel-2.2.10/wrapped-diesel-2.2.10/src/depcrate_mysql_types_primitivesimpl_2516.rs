// Generated macro for impl_2516 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2516 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2516"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < SmallInt , Mysql > for i16 { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { match value . numeric_value () ? { NumericRepresentation :: Tiny (x) => Ok (x . into ()) , NumericRepresentation :: Small (x) => Ok (x) , NumericRepresentation :: Medium (x) => x . try_into () . map_err (| _ | { Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _ }) , NumericRepresentation :: Big (x) => x . try_into () . map_err (| _ | { Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _ }) , NumericRepresentation :: Float (x) => f32_to_i64 (x) ? . try_into () . map_err (| _ | { Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _ }) , NumericRepresentation :: Double (x) => f64_to_i64 (x) ? . try_into () . map_err (| _ | { Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _ }) , NumericRepresentation :: Decimal (bytes) => decimal_to_integer (bytes) , } } }
};
}
