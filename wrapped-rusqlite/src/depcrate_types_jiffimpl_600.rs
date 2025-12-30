// Generated macro for impl_600 (impl)
macro_rules! Depcrate_types_jiffimpl_600 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_600"}
// Dependencies: {}
# [doc = " \"YYYY-MM-DDTHH:MM:SS.SSS\" => Gregorian datetime."] impl FromSql for DateTime { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | s . parse () . map_err (FromSqlError :: other)) } }
};
}
