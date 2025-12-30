// Generated macro for is_possibly_tzif (function)
macro_rules! Depcrate_tz_tzifis_possibly_tzif {
() => {
// Module: crate::tz::tzif
// Provides: {"is_possibly_tzif"}
// Dependencies: {}
# [doc = " Does a quick check that returns true if the data might be in TZif format."] # [doc = ""] # [doc = " It is possible that this returns true even if the given data is not in TZif"] # [doc = " format. However, it is impossible for this to return false when the given"] # [doc = " data is TZif. That is, a false positive is allowed but a false negative is"] # [doc = " not."] # [cfg (feature = "tzdb-zoneinfo")] pub (crate) fn is_possibly_tzif (data : & [u8]) -> bool { data . starts_with (b"TZif") }
};
}
