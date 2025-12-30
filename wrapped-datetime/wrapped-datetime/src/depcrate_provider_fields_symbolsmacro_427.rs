// Generated macro for macro_427 (macro)
macro_rules! Depcrate_provider_fields_symbolsmacro_427 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"macro_427"}
// Dependencies: {}
field_type ! (# [doc = " An enum for the possible symbols of a time zone field in a date pattern."] TimeZone ; { # [doc = " Field symbol for the specific non-location format of a time zone."] # [doc = ""] # [doc = " For example: \"Pacific Standard Time\""] 'z' => SpecificNonLocation = 0 , # [doc = " Field symbol for the localized offset format of a time zone."] # [doc = ""] # [doc = " For example: \"GMT-07:00\""] 'O' => LocalizedOffset = 1 , # [doc = " Field symbol for the generic non-location format of a time zone."] # [doc = ""] # [doc = " For example: \"Pacific Time\""] 'v' => GenericNonLocation = 2 , # [doc = " Field symbol for any of: the time zone id, time zone exemplar city, or generic location format."] 'V' => Location = 3 , # [doc = " Field symbol for either the ISO-8601 basic format or ISO-8601 extended format. This does not use an"] # [doc = " optional ISO-8601 UTC indicator `Z`, whereas [`TimeZone::IsoWithZ`] produces `Z`."] 'x' => Iso = 4 , # [doc = " Field symbol for either the ISO-8601 basic format or ISO-8601 extended format, with the ISO-8601 UTC indicator `Z`."] 'X' => IsoWithZ = 5 , } ; TimeZoneULE) ;
};
}
