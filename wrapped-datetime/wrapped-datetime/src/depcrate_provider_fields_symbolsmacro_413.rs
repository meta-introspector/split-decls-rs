// Generated macro for macro_413 (macro)
macro_rules! Depcrate_provider_fields_symbolsmacro_413 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"macro_413"}
// Dependencies: {}
field_type ! (# [doc = " An enum for the possible symbols of a year field in a date pattern."] Year ; { # [doc = " Field symbol for calendar year (numeric)."] # [doc = ""] # [doc = " In most cases the length of this field specifies the minimum number of digits to display, zero-padded as necessary. For most use cases, [`Year::Calendar`] or `Year::WeekOf` should be adequate."] 'y' => Calendar = 0 , # [doc = " Field symbol for cyclic year; used in calendars where years are tracked in cycles, such as the Chinese or Dangi calendars."] 'U' => Cyclic = 1 , # [doc = " Field symbol for related ISO; some calendars which use different year numbering than ISO, or no year numbering, may express years in an ISO year corresponding to a calendar year."] 'r' => RelatedIso = 2 , # [doc = " Field symbol for extended year"] 'u' => Extended = 3 , } ; YearULE) ;
};
}
