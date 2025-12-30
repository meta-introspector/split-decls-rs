// Generated macro for DateTimePatternFormatter (struct)
macro_rules! Depcrate_pattern_formatterDateTimePatternFormatter {
() => {
// Module: crate::pattern::formatter
// Provides: {"DateTimePatternFormatter"}
// Dependencies: {}
# [doc = " A formatter for a specific [`DateTimePattern`]."] # [doc = ""] # [doc = " ❗ This type forgoes most internationalization functionality of the datetime crate."] # [doc = " It assumes that the pattern is already localized for the customer's locale. Most clients"] # [doc = " should use [`DateTimeFormatter`] instead of directly formatting with patterns."] # [doc = ""] # [doc = " Create one of these via factory methods on [`FixedCalendarDateTimeNames`]."] # [doc = ""] # [doc = " [`DateTimePattern`]: super::DateTimePattern"] # [doc = " [`FixedCalendarDateTimeNames`]: super::FixedCalendarDateTimeNames"] # [doc = " [`DateTimeFormatter`]: crate::DateTimeFormatter"] # [derive (Debug , Copy , Clone)] pub struct DateTimePatternFormatter < 'a , C : CldrCalendar , FSet > { inner : RawDateTimePatternFormatter < 'a > , _calendar : PhantomData < C > , _marker : PhantomData < FSet > , }
};
}
