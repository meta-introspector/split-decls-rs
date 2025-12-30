// Generated macro for TimeZoneAnnotation (struct)
macro_rules! Depcrate_recordsTimeZoneAnnotation {
() => {
// Module: crate::records
// Provides: {"TimeZoneAnnotation"}
// Dependencies: {}
# [doc = " A `TimeZoneAnnotation` that represents a parsed `TimeZoneRecord` and its critical flag."] # [non_exhaustive] # [derive (Debug , Clone , PartialEq)] pub struct TimeZoneAnnotation < 'a , T : EncodingType > { # [doc = " Critical flag for the `TimeZoneAnnotation`."] pub critical : bool , # [doc = " The parsed `TimeZoneRecord` for the annotation."] pub tz : TimeZoneRecord < 'a , T > , }
};
}
