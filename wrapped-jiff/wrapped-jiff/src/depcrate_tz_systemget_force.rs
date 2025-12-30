// Generated macro for get_force (function)
macro_rules! Depcrate_tz_systemget_force {
() => {
// Module: crate::tz::system
// Provides: {"get_force"}
// Dependencies: {}
# [doc = " Always attempt retrieve the system time zone. This never uses a cache."] pub (crate) fn get_force (db : & TimeZoneDatabase) -> Result < TimeZone , Error > { match get_env_tz (db) { Ok (Some (tz)) => { debug ! ("checked TZ environment variable and found {tz:?}") ; return Ok (tz) ; } Ok (None) => { debug ! ("TZ environment variable is not set") ; } Err (err) => { return Err (err . context ("TZ environment variable set, but failed to read value" ,)) ; } } if let Some (tz) = sys :: get (db) { return Ok (tz) ; } Err (err ! ("failed to find system time zone")) }
};
}
