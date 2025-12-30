// Generated macro for impl_527 (impl)
macro_rules! Depcrate_types_chronoimpl_527 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_527"}
// Dependencies: {}
# [doc = " \"YYYY-MM-DD\" => ISO 8601 calendar date without timezone."] impl FromSql for NaiveDate { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | Self :: parse_from_str (s , "%F") . map_err (FromSqlError :: other)) } }
};
}
