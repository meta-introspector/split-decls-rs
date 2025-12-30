// Generated macro for impl_537 (impl)
macro_rules! Depcrate_types_chronoimpl_537 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_537"}
// Dependencies: {}
# [doc = " RFC3339 (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\") into `DateTime<FixedOffset>`."] impl FromSql for DateTime < FixedOffset > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let s = String :: column_result (value) ? ; Self :: parse_from_rfc3339 (s . as_str ()) . or_else (| _ | Self :: parse_from_str (s . as_str () , "%F %T%.f%:z")) . map_err (FromSqlError :: other) } }
};
}
