// Generated macro for impl_528 (impl)
macro_rules! Depcrate_types_chronoimpl_528 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_528"}
// Dependencies: {}
# [doc = " ISO 8601 time without timezone => \"HH:MM:SS.SSS\""] impl ToSql for NaiveTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%T%.f") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
