// Generated macro for impl_601 (impl)
macro_rules! Depcrate_types_jiffimpl_601 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_601"}
// Dependencies: {}
# [doc = " UTC time => UTC RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DDTHH:MM:SS.SSSZ\")."] impl ToSql for Timestamp { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . to_string ())) } }
};
}
