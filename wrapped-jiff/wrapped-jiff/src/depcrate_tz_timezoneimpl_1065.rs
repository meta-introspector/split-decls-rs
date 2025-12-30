// Generated macro for impl_1065 (impl)
macro_rules! Depcrate_tz_timezoneimpl_1065 {
() => {
// Module: crate::tz::timezone
// Provides: {"impl_1065"}
// Dependencies: {}
# [doc (hidden)] impl TimeZone { pub const fn __internal_from_tzif (tzif : & 'static crate :: tz :: tzif :: TzifStatic ,) -> TimeZone { let repr = Repr :: static_tzif (tzif) ; TimeZone { repr } } # [doc = " Returns a dumb copy of this `TimeZone`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that this time zone is UTC, unknown, a fixed"] # [doc = " offset or created with `TimeZone::__internal_from_tzif`."] # [doc = ""] # [doc = " Namely, this specifically does not increment the ref count for"] # [doc = " the `Arc` pointers when the tag is `ARC_TZIF` or `ARC_POSIX`."] # [doc = " This means that incorrect usage of this routine can lead to"] # [doc = " use-after-free."] # [inline] pub const unsafe fn copy (& self) -> TimeZone { unsafe { TimeZone { repr : self . repr . copy () } } } }
};
}
