// Generated macro for impl_595 (impl)
macro_rules! Depcrate_types_jiffimpl_595 {
() => {
// Module: crate::types::jiff
// Provides: {"impl_595"}
// Dependencies: {}
# [doc = " Gregorian calendar date => \"YYYY-MM-DD\""] impl ToSql for Date { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let s = self . to_string () ; Ok (ToSqlOutput :: from (s)) } }
};
}
