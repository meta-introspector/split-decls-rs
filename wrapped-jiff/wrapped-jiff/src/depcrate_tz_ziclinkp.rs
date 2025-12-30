// Generated macro for LinkP (struct)
macro_rules! Depcrate_tz_zicLinkP {
() => {
// Module: crate::tz::zic
// Provides: {"LinkP"}
// Dependencies: {}
# [doc = " The parsed representation of a `Link` line."] # [derive (Clone , Debug , Eq , PartialEq)] struct LinkP { # [doc = " The time zone being linked. This must be the name of some other `Zone`"] # [doc = " or `Link`."] target : ZoneNameP , # [doc = " The name of the link."] name : ZoneNameP , }
};
}
