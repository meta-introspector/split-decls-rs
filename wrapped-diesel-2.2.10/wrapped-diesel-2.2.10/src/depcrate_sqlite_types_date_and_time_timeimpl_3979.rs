// Generated macro for impl_3979 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3979 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3979"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl FromSql < Date , Sqlite > for NaiveDate { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { value . parse_string (| s | Self :: parse (s , DATE_FORMAT)) . map_err (Into :: into) } }
};
}
