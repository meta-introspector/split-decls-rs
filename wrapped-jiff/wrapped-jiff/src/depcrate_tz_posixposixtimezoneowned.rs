// Generated macro for PosixTimeZoneOwned (type)
macro_rules! Depcrate_tz_posixPosixTimeZoneOwned {
() => {
// Module: crate::tz::posix
// Provides: {"PosixTimeZoneOwned"}
// Dependencies: {}
# [doc = " An owned POSIX time zone."] # [doc = ""] # [doc = " That is, a POSIX time zone whose abbreviations are inlined into the"] # [doc = " representation. As opposed to a static POSIX time zone whose abbreviations"] # [doc = " are `&'static str`."] pub (crate) type PosixTimeZoneOwned = PosixTimeZone < Abbreviation > ;
};
}
