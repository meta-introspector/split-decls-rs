// Generated macro for TimeZoneRecord (enum)
macro_rules! Depcrate_recordsTimeZoneRecord {
() => {
// Module: crate::records
// Provides: {"TimeZoneRecord"}
// Dependencies: {}
# [doc = " Parsed `TimeZone` data, which can be either a UTC Offset value or IANA Time Zone Name value."] # [non_exhaustive] # [derive (Debug , Clone , PartialEq)] pub enum TimeZoneRecord < 'a , T : EncodingType > { # [doc = " TimeZoneIANAName"] Name (& 'a [T :: CodeUnit]) , # [doc = " TimeZoneOffset"] Offset (MinutePrecisionOffset) , }
};
}
