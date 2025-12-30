// Generated macro for test_zulu_offset (function)
macro_rules! Depcrate_parsers_teststest_zulu_offset {
() => {
// Module: crate::parsers::tests
// Provides: {"test_zulu_offset"}
// Dependencies: {}
# [test] fn test_zulu_offset () { let zdt = "2024-08-24T14:00:00Z[America/Chicago]" ; let result = IxdtfParser :: from_str (zdt) . parse () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : Some (DateRecord { year : 2024 , month : 8 , day : 24 , }) , time : Some (TimeRecord { hour : 14 , minute : 0 , second : 0 , fraction : None , }) , offset : Some (crate :: records :: UtcOffsetRecordOrZ :: Z) , tz : Some (TimeZoneAnnotation { critical : false , tz : TimeZoneRecord :: Name ("America/Chicago" . as_bytes ()) }) , calendar : None , })) ; let zdt = "2024-08-24T14:00:00Z" ; let result = IxdtfParser :: from_str (zdt) . parse () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : Some (DateRecord { year : 2024 , month : 8 , day : 24 , }) , time : Some (TimeRecord { hour : 14 , minute : 0 , second : 0 , fraction : None , }) , offset : Some (UtcOffsetRecordOrZ :: Z) , tz : None , calendar : None , })) ; }
};
}
