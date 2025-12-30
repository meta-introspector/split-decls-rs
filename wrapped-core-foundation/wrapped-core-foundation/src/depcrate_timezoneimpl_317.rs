// Generated macro for impl_317 (impl)
macro_rules! Depcrate_timezoneimpl_317 {
() => {
// Module: crate::timezone
// Provides: {"impl_317"}
// Dependencies: {}
impl CFTimeZone { # [inline] pub fn new (interval : CFTimeInterval) -> CFTimeZone { unsafe { let tz_ref = CFTimeZoneCreateWithTimeIntervalFromGMT (kCFAllocatorDefault , interval) ; TCFType :: wrap_under_create_rule (tz_ref) } } # [inline] pub fn system () -> CFTimeZone { unsafe { let tz_ref = CFTimeZoneCopySystem () ; TCFType :: wrap_under_create_rule (tz_ref) } } pub fn seconds_from_gmt (& self , date : CFDate) -> CFTimeInterval { unsafe { CFTimeZoneGetSecondsFromGMT (self . 0 , date . abs_time ()) } } # [doc = " The timezone database ID that identifies the time zone. E.g. `\"America/Los_Angeles\" `or"] # [doc = " `\"Europe/Paris\"`."] pub fn name (& self) -> CFString { unsafe { CFString :: wrap_under_get_rule (CFTimeZoneGetName (self . 0)) } } }
};
}
