// Generated macro for IxdtfParseRecord (struct)
macro_rules! Depcrate_recordsIxdtfParseRecord {
() => {
// Module: crate::records
// Provides: {"IxdtfParseRecord"}
// Dependencies: {}
# [doc = " An `IxdtfParseRecord` is an intermediary record returned by `IxdtfParser`."] # [non_exhaustive] # [derive (Default , Debug , PartialEq)] pub struct IxdtfParseRecord < 'a , T : EncodingType > { # [doc = " Parsed `DateRecord`"] pub date : Option < DateRecord > , # [doc = " Parsed `TimeRecord`"] pub time : Option < TimeRecord > , # [doc = " Parsed UtcOffset"] pub offset : Option < UtcOffsetRecordOrZ > , # [doc = " Parsed `TimeZone` annotation with critical flag and data (UTCOffset | IANA name)"] pub tz : Option < TimeZoneAnnotation < 'a , T > > , # [doc = " The parsed calendar value."] pub calendar : Option < & 'a [T :: CodeUnit] > , }
};
}
