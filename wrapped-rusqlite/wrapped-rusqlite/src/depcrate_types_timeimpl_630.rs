// Generated macro for impl_630 (impl)
macro_rules! Depcrate_types_timeimpl_630 {
() => {
// Module: crate::types::time
// Provides: {"impl_630"}
// Dependencies: {}
# [doc = " ISO 8601 time without timezone => \"HH:MM:SS.SSS\""] impl ToSql for Time { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let time_str = self . format (& TIME_ENCODING) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ? ; Ok (ToSqlOutput :: from (time_str)) } }
};
}
