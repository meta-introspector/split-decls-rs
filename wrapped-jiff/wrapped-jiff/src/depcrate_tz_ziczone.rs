// Generated macro for Zone (struct)
macro_rules! Depcrate_tz_zicZone {
() => {
// Module: crate::tz::zic
// Provides: {"Zone"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] struct Zone { # [doc = " The offset to add to UTC to get standard time for this time zone."] offset : Offset , # [doc = " The rules for determining the offset from standard time to use for this"] # [doc = " time zone."] rules : Rules , # [doc = " The format to use when rendering a time zone abbreviation."] format : ZoneFormatP , # [doc = " The timestamp until which this zone is active (exclusive)."] until_timestamp : Timestamp , # [doc = " The wall clock time until which this zone is active (exclusive)."] until_wall : DateTime , }
};
}
