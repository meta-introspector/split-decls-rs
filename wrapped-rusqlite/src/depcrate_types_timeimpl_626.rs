// Generated macro for impl_626 (impl)
macro_rules! Depcrate_types_timeimpl_626 {
() => {
// Module: crate::types::time
// Provides: {"impl_626"}
// Dependencies: {}
# [doc = " `OffsetDatetime` => RFC3339 format (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\")"] impl ToSql for OffsetDateTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let time_string = self . format (& OFFSET_DATE_TIME_ENCODING) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ? ; Ok (ToSqlOutput :: from (time_string)) } }
};
}
