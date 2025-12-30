// Generated macro for macro_417 (macro)
macro_rules! Depcrate_provider_fields_symbolsmacro_417 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"macro_417"}
// Dependencies: {}
field_type ! (# [doc = " An enum for the possible symbols of a day field in a date pattern."] Day ; { # [doc = " Field symbol for day of month (numeric)."] 'd' => DayOfMonth = 0 , # [doc = " Field symbol for day of year (numeric)."] 'D' => DayOfYear = 1 , # [doc = " Field symbol for the day of week occurrence relative to the month (numeric)."] # [doc = ""] # [doc = " For the example `\"2nd Wed in July\"`, this field would provide `\"2\"`.  Should likely be paired with the [`Weekday`] field."] 'F' => DayOfWeekInMonth = 2 , # [doc = " Field symbol for the modified Julian day (numeric)."] # [doc = ""] # [doc = " The value of this field differs from the conventional Julian day number in a couple of ways, which are based on measuring relative to the local time zone."] 'g' => ModifiedJulianDay = 3 , } ; Numeric ; DayULE) ;
};
}
