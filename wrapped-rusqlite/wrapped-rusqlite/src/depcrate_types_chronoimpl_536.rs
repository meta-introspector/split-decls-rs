// Generated macro for impl_536 (impl)
macro_rules! Depcrate_types_chronoimpl_536 {
() => {
// Module: crate::types::chrono
// Provides: {"impl_536"}
// Dependencies: {}
# [doc = " RFC3339 (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\") into `DateTime<Local>`."] impl FromSql for DateTime < Local > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let utc_dt = DateTime :: < Utc > :: column_result (value) ? ; Ok (utc_dt . with_timezone (& Local)) } }
};
}
