// Generated macro for DateTime (struct)
macro_rules! Depcrate_datetimeDateTime {
() => {
// Module: crate::datetime
// Provides: {"DateTime"}
// Dependencies: {}
# [doc = " Date-and-time type shared by multiple ASN.1 types"] # [doc = " (e.g. `GeneralizedTime`, `UTCTime`)."] # [doc = ""] # [doc = " Following conventions from RFC 5280, this type is always Z-normalized"] # [doc = " (i.e. represents a UTC time). However, it isn't named \"UTC time\" in order"] # [doc = " to prevent confusion with ASN.1 `UTCTime`."] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] pub struct DateTime { # [doc = " Full year (e.g. 2000)."] # [doc = ""] # [doc = " Must be >=1970 to permit positive conversions to Unix time."] year : u16 , # [doc = " Month (1-12)"] month : u8 , # [doc = " Day of the month (1-31)"] day : u8 , # [doc = " Hour (0-23)"] hour : u8 , # [doc = " Minutes (0-59)"] minutes : u8 , # [doc = " Seconds (0-59)"] seconds : u8 , # [doc = " [`Duration`] since the Unix epoch."] unix_duration : Duration , }
};
}
