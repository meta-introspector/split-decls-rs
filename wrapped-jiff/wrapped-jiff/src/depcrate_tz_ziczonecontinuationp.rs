// Generated macro for ZoneContinuationP (struct)
macro_rules! Depcrate_tz_zicZoneContinuationP {
() => {
// Module: crate::tz::zic
// Provides: {"ZoneContinuationP"}
// Dependencies: {}
# [doc = " The parser representation of a `Zone` line."] # [derive (Clone , Debug , Eq , PartialEq)] struct ZoneContinuationP { # [doc = " The offset to add to UTC to get \"standard\" time for this zone."] stdoff : ZoneStdoffP , # [doc = " The rules to apply for this zone. When `None`, standard time always"] # [doc = " applies."] rules : ZoneRulesP , # [doc = " The format to use when rendering a time zone abbreviation."] format : ZoneFormatP , # [doc = " The zone is active until this time."] # [doc = ""] # [doc = " When present, it implies the existence of a continuation zone following"] # [doc = " this one. When absent, it is the last zone line for a particular name."] until : Option < ZoneUntilP > , }
};
}
