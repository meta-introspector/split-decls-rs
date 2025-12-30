// Generated macro for TimeZoneInner (struct)
macro_rules! Depcrate_tz_zicTimeZoneInner {
() => {
// Module: crate::tz::zic
// Provides: {"TimeZoneInner"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] struct TimeZoneInner { # [doc = " The canonical name for a time zone, according to tzdb. There can only"] # [doc = " be one of these, and it is the name with which the `Zone` line used."] name : String , # [doc = " A list of zero or more aliases for this time zone created via `Link`"] # [doc = " lines."] aliases : Vec < String > , # [doc = " A sequence of one or more \"zones\" that make up this time zone. Each"] # [doc = " zone is active until a certain point in time, at which some other zone"] # [doc = " takes over. Each zone can reference a distinct set of rules, and thus"] # [doc = " the specific zone to use for any given time dictates how DST is"] # [doc = " computed for that time."] zones : Vec < Zone > , }
};
}
