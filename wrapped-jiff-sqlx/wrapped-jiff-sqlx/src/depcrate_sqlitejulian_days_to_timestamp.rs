// Generated macro for julian_days_to_timestamp (function)
macro_rules! Depcrate_sqlitejulian_days_to_timestamp {
() => {
// Module: crate::sqlite
// Provides: {"julian_days_to_timestamp"}
// Dependencies: {}
fn julian_days_to_timestamp (days : f64 ,) -> Result < jiff :: Timestamp , BoxDynError > { static UNIX_EPOCH_AS_JULIAN_DAYS : f64 = 2440587.5 ; static SECONDS_PER_DAY : f64 = 86400.0 ; let timestamp = (days - UNIX_EPOCH_AS_JULIAN_DAYS) * SECONDS_PER_DAY ; let sdur = jiff :: SignedDuration :: try_from_secs_f64 (timestamp) ? ; Ok (jiff :: Timestamp :: from_duration (sdur) ?) }
};
}
