// Generated macro for macro_423 (macro)
macro_rules! Depcrate_provider_fields_symbolsmacro_423 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"macro_423"}
// Dependencies: {}
field_type ! (# [doc = " An enum for the possible symbols of a weekday field in a date pattern."] Weekday ; { # [doc = " Field symbol for day of week (text format only)."] 'E' => Format = 0 , # [doc = " Field symbol for day of week; numeric formats produce a locale-dependent ordinal weekday number."] # [doc = ""] # [doc = " For example, in de-DE, Monday is the 1st day of the week."] 'e' => Local = 1 , # [doc = " Field symbol for stand-alone local day of week number/name."] # [doc = ""] # [doc = " The stand-alone weekday name is used when the weekday is displayed by itself. This may differ from the standard form based on the language and context."] 'c' => StandAlone = 2 , } ; WeekdayULE) ;
};
}
