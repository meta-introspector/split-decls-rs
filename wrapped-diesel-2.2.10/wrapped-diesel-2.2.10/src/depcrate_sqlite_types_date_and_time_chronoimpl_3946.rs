// Generated macro for impl_3946 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3946 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3946"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl FromSql < Date , Sqlite > for NaiveDate { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { value . parse_string (| s | Self :: parse_from_str (s , DATE_FORMAT)) . map_err (Into :: into) } }
};
}
