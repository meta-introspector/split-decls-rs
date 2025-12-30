// Generated macro for Numeric (struct)
macro_rules! Depcrate_fmt_offsetNumeric {
() => {
// Module: crate::fmt::offset
// Provides: {"Numeric"}
// Dependencies: {}
# [doc = " A numeric representation of a UTC offset."] struct Numeric { # [doc = " The sign that was parsed from the numeric UTC offset. This is always"] # [doc = " either `1` or `-1`, never `0`."] sign : t :: Sign , # [doc = " The hours component. This is non-optional because every UTC offset must"] # [doc = " have at least hours."] hours : ParsedOffsetHours , # [doc = " The minutes component."] minutes : Option < ParsedOffsetMinutes > , # [doc = " The seconds component. This is only possible when subminute resolution"] # [doc = " is enabled."] seconds : Option < ParsedOffsetSeconds > , # [doc = " The nanoseconds fractional component. This is only possible when"] # [doc = " subminute resolution is enabled."] nanoseconds : Option < t :: SubsecNanosecond > , }
};
}
