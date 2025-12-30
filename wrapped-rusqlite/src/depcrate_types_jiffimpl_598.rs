// Generated macro for impl_598 (impl)
macro_rules! Depcrate_types_jiffimpl_598 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_598"}
// Dependencies: {}
# [doc = " \"HH:MM:SS.SSS\" => time."] impl FromSql for Time { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | s . parse () . map_err (FromSqlError :: other)) } }
};
}
