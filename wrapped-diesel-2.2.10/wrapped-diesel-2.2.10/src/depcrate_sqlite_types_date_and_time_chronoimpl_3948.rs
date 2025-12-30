// Generated macro for impl_3948 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3948 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3948"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl FromSql < Time , Sqlite > for NaiveTime { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { value . parse_string (| text | { for format in TIME_FORMATS { if let Ok (time) = Self :: parse_from_str (text , format) { return Ok (time) ; } } Err (format ! ("Invalid time {text}") . into ()) }) } }
};
}
