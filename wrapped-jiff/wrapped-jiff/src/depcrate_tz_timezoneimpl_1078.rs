// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_tz_timezoneimpl_1078 {
() => {
// Module: crate::tz::timezone
// Provides: {"impl_1078"}
// Dependencies: {}
impl < 'a > core :: fmt :: Display for DiagnosticName < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { repr :: each ! { & self . 0 . repr , UTC => write ! (f , "UTC") , UNKNOWN => write ! (f , "Etc/Unknown") , FIXED (offset) => write ! (f , "{offset}") , STATIC_TZIF (tzif) => write ! (f , "{}" , tzif . name () . unwrap_or ("Local")) , ARC_TZIF (tzif) => write ! (f , "{}" , tzif . name () . unwrap_or ("Local")) , ARC_POSIX (posix) => write ! (f , "{posix}") , } } }
};
}
