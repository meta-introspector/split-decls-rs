// Generated macro for impl_602 (impl)
macro_rules! Depcrate_types_jiffimpl_602 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_602"}
// Dependencies: {}
# [doc = " RFC3339 (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\") into `Timestamp`."] impl FromSql for Timestamp { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () ? . parse :: < Timestamp > () . map_err (FromSqlError :: other) } }
};
}
