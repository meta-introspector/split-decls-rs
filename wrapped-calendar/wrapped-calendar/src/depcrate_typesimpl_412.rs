// Generated macro for impl_412 (impl)
macro_rules! Depcrate_typesimpl_412 {
() => {
// Module: crate::types
// Provides: {"impl_412"}
// Dependencies: {}
impl Weekday { # [doc = " Convert from an ISO-8601 weekday number to an [`Weekday`] enum. 0 is automatically converted"] # [doc = " to 7 (Sunday). If the number is out of range, it is interpreted modulo 7."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::calendar::types::Weekday;"] # [doc = ""] # [doc = " assert_eq!(Weekday::Sunday, Weekday::from_days_since_sunday(0));"] # [doc = " assert_eq!(Weekday::Monday, Weekday::from_days_since_sunday(1));"] # [doc = " assert_eq!(Weekday::Sunday, Weekday::from_days_since_sunday(7));"] # [doc = " assert_eq!(Weekday::Monday, Weekday::from_days_since_sunday(8));"] # [doc = " ```"] pub fn from_days_since_sunday (input : isize) -> Self { (SUNDAY + input as i64) . into () } # [doc = " Returns the day after the current day."] pub (crate) fn next_day (self) -> Weekday { use Weekday :: * ; match self { Monday => Tuesday , Tuesday => Wednesday , Wednesday => Thursday , Thursday => Friday , Friday => Saturday , Saturday => Sunday , Sunday => Monday , } } }
};
}
