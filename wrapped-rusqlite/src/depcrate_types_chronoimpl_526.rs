// Generated macro for impl_526 (impl)
macro_rules! Depcrate_types_chronoimpl_526 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_526"}
// Dependencies: {}
# [doc = " ISO 8601 calendar date without timezone => \"YYYY-MM-DD\""] impl ToSql for NaiveDate { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
