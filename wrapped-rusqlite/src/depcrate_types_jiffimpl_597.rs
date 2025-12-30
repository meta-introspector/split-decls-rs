// Generated macro for impl_597 (impl)
macro_rules! Depcrate_types_jiffimpl_597 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_597"}
// Dependencies: {}
# [doc = " time => \"HH:MM:SS.SSS\""] impl ToSql for Time { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
