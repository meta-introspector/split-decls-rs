// Generated macro for impl_596 (impl)
macro_rules! Depcrate_types_jiffimpl_596 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_596"}
// Dependencies: {}
# [doc = " \"YYYY-MM-DD\" => Gregorian calendar date."] impl FromSql for Date { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | s . parse () . map_err (FromSqlError :: other)) } }
};
}
