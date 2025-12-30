// Generated macro for impl_1120 (impl)
macro_rules! Depcrate_tz_zicimpl_1120 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1120"}
// Dependencies: {}
impl ZoneContinuationP { fn parse (fields : & [& str]) -> Result < ZoneContinuationP , Error > { if fields . len () < 3 { return Err (err ! ("continuation ZONE line must have at least 3 fields")) ; } let (stdoff_field , fields) = (fields [0] , & fields [1 ..]) ; let (rules_field , fields) = (fields [0] , & fields [1 ..]) ; let (format_field , fields) = (fields [0] , & fields [1 ..]) ; let stdoff = stdoff_field . parse :: < ZoneStdoffP > () . map_err (| e | e . context ("failed to parse STDOFF field")) ? ; let rules = rules_field . parse :: < ZoneRulesP > () . map_err (| e | e . context ("failed to parse RULES field")) ? ; let format = format_field . parse :: < ZoneFormatP > () . map_err (| e | e . context ("failed to parse FORMAT field")) ? ; let until = if fields . is_empty () { None } else { Some (ZoneUntilP :: parse (fields) . map_err (| e | e . context ("failed to parse UNTIL field")) ? ,) } ; Ok (ZoneContinuationP { stdoff , rules , format , until }) } }
};
}
