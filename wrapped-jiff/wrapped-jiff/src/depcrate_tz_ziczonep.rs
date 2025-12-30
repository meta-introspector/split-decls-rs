// Generated macro for ZoneP (struct)
macro_rules! Depcrate_tz_zicZoneP {
() => {
// Module: crate::tz::zic
// Provides: {"ZoneP"}
// Dependencies: {}
# [doc = " A group of one or more `Zone` lines."] # [doc = ""] # [doc = " A group of zones always starts with a `Zone` line that has a name, and is"] # [doc = " followed by zero or more continuation `Zone` lines."] # [derive (Clone , Debug , Eq , PartialEq)] struct ZoneP { # [doc = " The first zone line, always present."] first : ZoneFirstP , # [doc = " All continuation lines, may be empty."] continuations : Vec < ZoneContinuationP > , }
};
}
