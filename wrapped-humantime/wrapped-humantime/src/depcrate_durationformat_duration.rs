// Generated macro for format_duration (function)
macro_rules! Depcrate_durationformat_duration {
() => {
// Module: crate::duration
// Provides: {"format_duration"}
// Dependencies: {}
# [doc = " Formats duration into a human-readable string"] # [doc = ""] # [doc = " Note: this format is guaranteed to have same value when using"] # [doc = " parse_duration, but we can change some details of the exact composition"] # [doc = " of the value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = " use humantime::format_duration;"] # [doc = ""] # [doc = " let val1 = Duration::new(9420, 0);"] # [doc = " assert_eq!(format_duration(val1).to_string(), \"2h 37m\");"] # [doc = " let val2 = Duration::new(0, 32_000_000);"] # [doc = " assert_eq!(format_duration(val2).to_string(), \"32ms\");"] # [doc = " ```"] pub fn format_duration (val : Duration) -> FormattedDuration { FormattedDuration (val) }
};
}
