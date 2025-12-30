// Generated macro for ZoneInfo (struct)
macro_rules! Depcrate_lineZoneInfo {
() => {
// Module: crate::line
// Provides: {"ZoneInfo"}
// Dependencies: {}
# [doc = " The information contained in both zone lines *and* zone continuation lines."] # [derive (PartialEq , Debug , Copy , Clone)] pub struct ZoneInfo < 'a > { # [doc = " The amount of time that needs to be added to UTC to get the standard"] # [doc = " time in this zone."] pub utc_offset : TimeSpec , # [doc = " The name of all the rules that should apply in the time zone, or the"] # [doc = " amount of time to add."] pub saving : Saving < 'a > , # [doc = " The format for time zone abbreviations, with `%s` as the string marker."] pub format : & 'a str , # [doc = " The time at which the rules change for this location, or `None` if"] # [doc = " these rules are in effect until the end of time (!)."] pub time : Option < ChangeTime > , }
};
}
