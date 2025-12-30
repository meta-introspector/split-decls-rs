// Generated macro for ZoneInfo (struct)
macro_rules! Depcrate_tableZoneInfo {
() => {
// Module: crate::table
// Provides: {"ZoneInfo"}
// Dependencies: {}
# [doc = " An owned zone definition line."] # [doc = ""] # [doc = " This struct mimics the `ZoneInfo` struct in the `line` module, *not* the"] # [doc = " `Zone` struct, which is the key name in the map—this is just the value."] # [doc = ""] # [doc = " As with `RuleInfo`, this struct uses owned Strings rather than string"] # [doc = " slices."] # [derive (PartialEq , Debug)] pub struct ZoneInfo { # [doc = " The number of seconds that need to be added to UTC to get the"] # [doc = " standard time in this zone."] pub offset : i64 , # [doc = " The name of all the rules that should apply in the time zone, or the"] # [doc = " amount of daylight-saving time to add."] pub saving : Saving , # [doc = " The format for time zone abbreviations."] pub format : Format , # [doc = " The time at which the rules change for this time zone, or `None` if"] # [doc = " these rules are in effect until the end of time (!)."] pub end_time : Option < ChangeTime > , }
};
}
