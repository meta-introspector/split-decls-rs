// Generated macro for impl_316 (impl)
macro_rules! Depcrate_timezoneimpl_316 {
() => {
// Module: crate::timezone
// Provides: {"impl_316"}
// Dependencies: {}
impl Default for CFTimeZone { fn default () -> CFTimeZone { unsafe { let tz_ref = CFTimeZoneCopyDefault () ; TCFType :: wrap_under_create_rule (tz_ref) } } }
};
}
