// Generated macro for impl_599 (impl)
macro_rules! Depcrate_types_jiffimpl_599 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_599"}
// Dependencies: {}
# [doc = " Gregorian datetime => \"YYYY-MM-DDTHH:MM:SS.SSS\""] impl ToSql for DateTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let s = self . to_string () ; Ok (ToSqlOutput :: from (s)) } }
};
}
