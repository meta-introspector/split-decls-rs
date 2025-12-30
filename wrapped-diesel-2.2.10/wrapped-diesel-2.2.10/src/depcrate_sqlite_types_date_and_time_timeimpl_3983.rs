// Generated macro for impl_3983 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3983 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3983"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl FromSql < Timestamp , Sqlite > for PrimitiveDateTime { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { value . parse_string (| text | { for format in PRIMITIVE_DATETIME_FORMATS { if let Ok (dt) = Self :: parse (text , format) { return Ok (dt) ; } } if let Ok (julian_days) = text . parse :: < f64 > () { if let Ok (timestamp) = parse_julian (julian_days) { return Ok (timestamp) ; } } Err (format ! ("Invalid datetime {text}") . into ()) }) } }
};
}
