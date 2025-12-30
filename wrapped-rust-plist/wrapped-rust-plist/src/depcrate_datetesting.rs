// Generated macro for testing (module)
macro_rules! Depcrate_datetesting {
() => {
// Module: crate::date
// Provides: {"testing"}
// Dependencies: {}
# [cfg (test)] mod testing { use super :: * ; # [test] fn date_roundtrip () { let date_str = "1981-05-16T11:32:06Z" ; let date = Date :: from_xml_format (date_str) . expect ("should parse") ; let generated_str = date . to_xml_format () ; assert_eq ! (date_str , generated_str) ; } # [test] fn far_past_date () { let date_str = "1920-01-01T00:00:00Z" ; Date :: from_xml_format (date_str) . expect ("should parse") ; } }
};
}
