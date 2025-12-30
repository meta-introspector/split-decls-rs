// Generated macro for utf16_basic_test (function)
macro_rules! Depcrate_parsers_testsutf16_basic_test {
() => {
// Module: crate::parsers::tests
// Provides: {"utf16_basic_test"}
// Dependencies: {}
# [test] fn utf16_basic_test () { let utf16 : Vec < u16 > = "2020-04-08[America/Chicago]" . as_bytes () . iter () . copied () . map (u16 :: from) . collect () ; let result = IxdtfParser :: < Utf16 > :: new (& utf16) . parse () ; let id = match result { Ok (IxdtfParseRecord { date : Some (DateRecord { year : 2020 , month : 4 , day : 8 , }) , time : None , offset : None , tz : Some (TimeZoneAnnotation { critical : false , tz : TimeZoneRecord :: Name (id) , }) , calendar : None , }) => id , _ => unreachable ! () , } ; assert_eq ! (String :: from_utf16_lossy (id) , "America/Chicago") ; }
};
}
