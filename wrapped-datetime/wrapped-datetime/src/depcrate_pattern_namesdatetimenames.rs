// Generated macro for DateTimeNames (struct)
macro_rules! Depcrate_pattern_namesDateTimeNames {
() => {
// Module: crate::pattern::names
// Provides: {"DateTimeNames"}
// Dependencies: {}
# [doc = " A low-level type that formats datetime patterns with localized names."] # [doc = " The calendar is chosen in the constructor at runtime."] # [doc = ""] # [doc = " Currently this only supports loading of non-calendar-specific names, but"] # [doc = " additional functions may be added in the future. If you need this, see"] # [doc = " <https://github.com/unicode-org/icu4x/issues/6107>"] # [derive (Debug , Clone)] pub struct DateTimeNames < FSet : DateTimeNamesMarker > { inner : FixedCalendarDateTimeNames < () , FSet > , calendar : FormattableAnyCalendar , }
};
}
