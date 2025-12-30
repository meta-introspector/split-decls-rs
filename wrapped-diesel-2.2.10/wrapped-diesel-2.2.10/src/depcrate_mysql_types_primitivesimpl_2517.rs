// Generated macro for impl_2517 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2517 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2517"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl FromSql < Integer , Mysql > for i32 { fn from_sql (value : MysqlValue < '_ >) -> deserialize :: Result < Self > { match value . numeric_value () ? { NumericRepresentation :: Tiny (x) => Ok (x . into ()) , NumericRepresentation :: Small (x) => Ok (x . into ()) , NumericRepresentation :: Medium (x) => Ok (x) , NumericRepresentation :: Big (x) => x . try_into () . map_err (| _ | { Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _ }) , NumericRepresentation :: Float (x) => f32_to_i64 (x) . and_then (| i | { i . try_into () . map_err (| _ | { Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _ }) }) , NumericRepresentation :: Double (x) => f64_to_i64 (x) . and_then (| i | { i . try_into () . map_err (| _ | { Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _ }) }) , NumericRepresentation :: Decimal (bytes) => decimal_to_integer (bytes) , } } }
};
}
