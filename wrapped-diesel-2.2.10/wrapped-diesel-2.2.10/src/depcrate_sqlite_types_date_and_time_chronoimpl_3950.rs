// Generated macro for impl_3950 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoimpl_3950 {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"impl_3950"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "chrono"))] impl FromSql < Timestamp , Sqlite > for NaiveDateTime { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { value . parse_string (| text | { for format in NAIVE_DATETIME_FORMATS { if let Ok (dt) = Self :: parse_from_str (text , format) { return Ok (dt) ; } } if let Ok (julian_days) = text . parse :: < f64 > () { if let Some (timestamp) = parse_julian (julian_days) { return Ok (timestamp) ; } } Err (format ! ("Invalid datetime {text}") . into ()) }) } }
};
}
