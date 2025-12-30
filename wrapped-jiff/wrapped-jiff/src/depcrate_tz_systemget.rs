// Generated macro for get (function)
macro_rules! Depcrate_tz_systemget {
() => {
// Module: crate::tz::system
// Provides: {"get"}
// Dependencies: {}
# [doc = " Retrieve the \"system\" time zone."] # [doc = ""] # [doc = " If there is a cached time zone that isn't stale, then that is returned"] # [doc = " instead."] # [doc = ""] # [doc = " If there is no cached time zone, then this tries to determine the system"] # [doc = " time zone in a platform specific manner. This may involve reading files"] # [doc = " or making system calls. If that fails then an error is returned."] # [doc = ""] # [doc = " Note that the `TimeZone` returned may not have an IANA name! In some cases,"] # [doc = " it is just impractical to determine the time zone name. For example, when"] # [doc = " `/etc/localtime` is a hard link to a TZif file instead of a symlink and"] # [doc = " when the time zone name isn't recorded in any of the other obvious places."] pub (crate) fn get (db : & TimeZoneDatabase) -> Result < TimeZone , Error > { { let cache = CACHE . read () . unwrap () ; if let Some (ref tz) = cache . tz { if ! cache . expiration . is_expired () { return Ok (tz . clone ()) ; } } } let tz = get_force (db) ? ; { let mut cache = CACHE . write () . unwrap () ; cache . tz = Some (tz . clone ()) ; cache . expiration = Expiration :: after (TTL) ; } Ok (tz) }
};
}
