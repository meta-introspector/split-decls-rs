// Generated macro for impl_633 (impl)
macro_rules! Depcrate_types_timeimpl_633 {
() => {
// Module: crate::types::time
// Provides: {"impl_633"}
// Dependencies: {}
# [doc = " YYYY-MM-DD HH:MM"] # [doc = " YYYY-MM-DDTHH:MM"] # [doc = " YYYY-MM-DD HH:MM:SS"] # [doc = " YYYY-MM-DDTHH:MM:SS"] # [doc = " YYYY-MM-DD HH:MM:SS.SSS"] # [doc = " YYYY-MM-DDTHH:MM:SS.SSS"] # [doc = " => ISO 8601 combined date and time with timezone"] impl FromSql for PrimitiveDateTime { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { Self :: parse (s , & PRIMITIVE_DATE_TIME_FORMAT) . map_err (| err | FromSqlError :: Other (err . into ())) }) } }
};
}
