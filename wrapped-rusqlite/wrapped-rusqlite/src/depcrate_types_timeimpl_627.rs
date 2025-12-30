// Generated macro for impl_627 (impl)
macro_rules! Depcrate_types_timeimpl_627 {
() => {
// Module: crate::types::time
// Provides: {"impl_627"}
// Dependencies: {}
impl FromSql for OffsetDateTime { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { if let Some (b' ') = s . as_bytes () . get (23) { return Self :: parse (s , & LEGACY_DATE_TIME_FORMAT) . map_err (FromSqlError :: other) ; } if s [8 ..] . contains ('+') || s [8 ..] . contains ('-') { return Self :: parse (s , & OFFSET_DATE_TIME_FORMAT) . map_err (FromSqlError :: other) ; } PrimitiveDateTime :: parse (s , & UTC_DATE_TIME_FORMAT) . map (| p | p . assume_utc ()) . map_err (FromSqlError :: other) }) } }
};
}
