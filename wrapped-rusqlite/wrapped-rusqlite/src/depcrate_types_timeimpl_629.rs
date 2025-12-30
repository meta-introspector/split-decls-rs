// Generated macro for impl_629 (impl)
macro_rules! Depcrate_types_timeimpl_629 {
() => {
// Module: crate::types::time
// Provides: {"impl_629"}
// Dependencies: {}
# [doc = " \"YYYY-MM-DD\" => ISO 8601 calendar date without timezone."] impl FromSql for Date { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { Self :: parse (s , & DATE_FORMAT) . map_err (| err | FromSqlError :: Other (err . into ())) }) } }
};
}
