// Generated macro for ZoneFormatP (enum)
macro_rules! Depcrate_tz_zicZoneFormatP {
() => {
// Module: crate::tz::zic
// Provides: {"ZoneFormatP"}
// Dependencies: {}
# [doc = " The format of the abbreviation for this zone, including when it's in DST."] # [derive (Clone , Debug , Eq , PartialEq)] enum ZoneFormatP { # [doc = " This corresponds to a format that contains a `%s`, where the `%s` is"] # [doc = " meant to be interpolated with the letters specified in a matching rule."] Variable { # [doc = " The text before a `%s`."] before : String , # [doc = " The text after a `%s`."] after : String , } , # [doc = " This corresponds to the `%z` format, where the abbreviation should"] # [doc = " be a human readable rendering of the offset applied."] Offset , # [doc = " This corresponds to the `STD/DST` format, where `STD` is the abbreviation"] # [doc = " for standard time, and `DST` is the abbreviation for DST."] Pair { # [doc = " The abbreviation to use for standard time."] std : String , # [doc = " The abbreviation to use for DST time."] dst : String , } , # [doc = " A static string that never changes."] Static { # [doc = " The format string which is never interpolated."] format : String , } , }
};
}
