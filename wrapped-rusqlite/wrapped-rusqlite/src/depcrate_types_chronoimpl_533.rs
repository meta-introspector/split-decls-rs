// Generated macro for impl_533 (impl)
macro_rules! Depcrate_types_chronoimpl_533 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_533"}
// Dependencies: {}
# [doc = " Local time => UTC RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DD HH:MM:SS.SSS+00:00\")."] impl ToSql for DateTime < Local > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . with_timezone (& Utc) . format ("%F %T%.f%:z") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
