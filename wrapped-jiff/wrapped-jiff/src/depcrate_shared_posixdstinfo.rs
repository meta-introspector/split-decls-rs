// Generated macro for DstInfo (struct)
macro_rules! Depcrate_shared_posixDstInfo {
() => {
// Module: crate::shared::posix
// Provides: {"DstInfo"}
// Dependencies: {}
# [doc = " The daylight saving time (DST) info for a POSIX time zone in a particular"] # [doc = " year."] # [derive (Debug , Eq , PartialEq)] struct DstInfo < 'a , ABBREV > { # [doc = " The DST transition rule that generated this info."] dst : & 'a PosixDst < ABBREV > , # [doc = " The start time (inclusive) that DST begins."] # [doc = ""] # [doc = " Note that this may be greater than `end`. This tends to happen in the"] # [doc = " southern hemisphere."] # [doc = ""] # [doc = " Note also that this may be in UTC or in wall clock civil"] # [doc = " time. It depends on whether `PosixTimeZone::dst_info_utc` or"] # [doc = " `PosixTimeZone::dst_info_wall` was used."] start : IDateTime , # [doc = " The end time (exclusive) that DST ends."] # [doc = ""] # [doc = " Note that this may be less than `start`. This tends to happen in the"] # [doc = " southern hemisphere."] # [doc = ""] # [doc = " Note also that this may be in UTC or in wall clock civil"] # [doc = " time. It depends on whether `PosixTimeZone::dst_info_utc` or"] # [doc = " `PosixTimeZone::dst_info_wall` was used."] end : IDateTime , }
};
}
