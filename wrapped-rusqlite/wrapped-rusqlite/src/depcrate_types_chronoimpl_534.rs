// Generated macro for impl_534 (impl)
macro_rules! Depcrate_types_chronoimpl_534 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_534"}
// Dependencies: {}
# [doc = " Date and time with time zone => RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\")."] impl ToSql for DateTime < FixedOffset > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F %T%.f%:z") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
