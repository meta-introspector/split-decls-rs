// Generated macro for impl_3981 (impl)
macro_rules! Depcrate_sqlite_types_date_and_time_timeimpl_3981 {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"impl_3981"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "time"))] impl FromSql < Time , Sqlite > for NaiveTime { fn from_sql (mut value : < Sqlite as Backend > :: RawValue < '_ >) -> deserialize :: Result < Self > { value . parse_string (| text | { for format in TIME_FORMATS { if let Ok (time) = Self :: parse (text , format) { return Ok (time) ; } } Err (format ! ("Invalid time {text}") . into ()) }) } }
};
}
