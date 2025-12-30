// Generated macro for DateTimeFormatter (struct)
macro_rules! Depcrate_neoDateTimeFormatter {
() => {
// Module: crate::neo
// Provides: {"DateTimeFormatter"}
// Dependencies: {}
# [doc = " [`DateTimeFormatter`] is a formatter capable of formatting dates and/or times from"] # [doc = " a calendar selected at runtime."] # [doc = ""] # [doc = " For more details, please read the [crate root docs][crate]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::datetime::fieldsets::YMD;"] # [doc = " use icu::datetime::input::Date;"] # [doc = " use icu::datetime::DateTimeFormatter;"] # [doc = " use icu::locale::locale;"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " let formatter = DateTimeFormatter::try_new("] # [doc = "     locale!(\"en-u-ca-hebrew\").into(),"] # [doc = "     YMD::medium(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " let date = Date::try_new_iso(2024, 5, 8).unwrap();"] # [doc = ""] # [doc = " assert_writeable_eq!(formatter.format(&date), \"30 Nisan 5784\");"] # [doc = " ```"] # [doc = neo_year_month_day_formatter_size ! ()] # [derive (Debug , Clone)] pub struct DateTimeFormatter < FSet : DateTimeNamesMarker > { pub (crate) selection : DateTimeZonePatternSelectionData , pub (crate) names : RawDateTimeNames < FSet > , pub (crate) calendar : UntaggedFormattableAnyCalendar , }
};
}
