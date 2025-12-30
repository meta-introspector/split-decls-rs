// Generated macro for impl_532 (impl)
macro_rules! Depcrate_types_chronoimpl_532 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_532"}
// Dependencies: {}
# [doc = " UTC time => UTC RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DD HH:MM:SS.SSS+00:00\")."] impl ToSql for DateTime < Utc > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F %T%.f%:z") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
