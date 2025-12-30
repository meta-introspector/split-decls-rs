// Generated macro for impl_1118 (impl)
macro_rules! Depcrate_tz_zicimpl_1118 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1118"}
// Dependencies: {}
impl ZoneFirstP { fn parse (fields : & [& str]) -> Result < ZoneFirstP , Error > { if fields . len () < 4 { return Err (err ! ("first ZONE line must have at least 4 fields")) ; } let (name_field , fields) = (fields [0] , & fields [1 ..]) ; let (stdoff_field , fields) = (fields [0] , & fields [1 ..]) ; let (rules_field , fields) = (fields [0] , & fields [1 ..]) ; let (format_field , fields) = (fields [0] , & fields [1 ..]) ; let name = name_field . parse :: < ZoneNameP > () . map_err (| e | e . context ("failed to parse NAME field")) ? ; let stdoff = stdoff_field . parse :: < ZoneStdoffP > () . map_err (| e | e . context ("failed to parse STDOFF field")) ? ; let rules = rules_field . parse :: < ZoneRulesP > () . map_err (| e | e . context ("failed to parse RULES field")) ? ; let format = format_field . parse :: < ZoneFormatP > () . map_err (| e | e . context ("failed to parse FORMAT field")) ? ; let until = if fields . is_empty () { None } else { Some (ZoneUntilP :: parse (fields) . map_err (| e | e . context ("failed to parse UNTIL field")) ? ,) } ; Ok (ZoneFirstP { name , stdoff , rules , format , until }) } }
};
}
