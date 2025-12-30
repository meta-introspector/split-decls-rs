// Generated macro for impl_530 (impl)
macro_rules! Depcrate_types_chronoimpl_530 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_530"}
// Dependencies: {}
# [doc = " ISO 8601 combined date and time without timezone =>"] # [doc = " \"YYYY-MM-DD HH:MM:SS.SSS\""] impl ToSql for NaiveDateTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F %T%.f") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
};
}
