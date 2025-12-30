// Generated macro for impl_39 (impl)
macro_rules! Depcrate_sqliteimpl_39 {
() => {
// Module: crate::sqlite
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'r > Decode < 'r , Sqlite > for Timestamp { fn decode (value : SqliteValueRef < 'r >) -> Result < Self , BoxDynError > { let text = < & str as Decode < Sqlite > > :: decode (value) ? ; if text . contains (':') { let date = PARSER . parse_timestamp (text) ? ; return Ok (date . to_sqlx ()) ; } let julian_days = text . parse :: < f64 > () ? ; julian_days_to_timestamp (julian_days) . map (jiff :: Timestamp :: to_sqlx) } }
};
}
