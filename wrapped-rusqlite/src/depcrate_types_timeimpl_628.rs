// Generated macro for impl_628 (impl)
macro_rules! Depcrate_types_timeimpl_628 {
() => {
// Module: crate::types::time
// Provides: {"impl_628"}
// Dependencies: {}
# [doc = " ISO 8601 calendar date without timezone => \"YYYY-MM-DD\""] impl ToSql for Date { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format (& DATE_FORMAT) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ? ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
