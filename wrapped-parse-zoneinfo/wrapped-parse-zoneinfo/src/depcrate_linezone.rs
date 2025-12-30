// Generated macro for Zone (struct)
macro_rules! Depcrate_lineZone {
() => {
// Module: crate::line
// Provides: {"Zone"}
// Dependencies: {}
# [doc = " A **zone** definition line."] # [doc = ""] # [doc = " According to the `zic(8)` man page, a zone line has this form, along with"] # [doc = " an example:"] # [doc = ""] # [doc = " ```text"] # [doc = "     Zone  NAME                GMTOFF  RULES/SAVE  FORMAT  [UNTILYEAR [MONTH [DAY [TIME]]]]"] # [doc = "     Zone  Australia/Adelaide  9:30    Aus         AC%sT   1971       Oct    31   2:00"] # [doc = " ```"] # [doc = ""] # [doc = " The opening `Zone` identifier is ignored, and the last four columns are"] # [doc = " all optional, with their variants consolidated into a `ChangeTime`."] # [doc = ""] # [doc = " The `Rules/Save` column, if it contains a value, *either* contains the"] # [doc = " name of the rules to use for this zone, *or* contains a one-off period of"] # [doc = " time to save."] # [doc = ""] # [doc = " A continuation rule line contains all the same fields apart from the"] # [doc = " `Name` column and the opening `Zone` identifier."] # [derive (PartialEq , Debug , Copy , Clone)] pub struct Zone < 'a > { # [doc = " The name of the time zone."] pub name : & 'a str , # [doc = " All the other fields of info."] pub info : ZoneInfo < 'a > , }
};
}
