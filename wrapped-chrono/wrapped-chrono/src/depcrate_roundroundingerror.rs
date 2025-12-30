// Generated macro for RoundingError (enum)
macro_rules! Depcrate_roundRoundingError {
() => {
// Module: crate::round
// Provides: {"RoundingError"}
// Dependencies: {}
# [doc = " An error from rounding by `TimeDelta`"] # [doc = ""] # [doc = " See: [`DurationRound`]"] # [derive (Debug , Clone , PartialEq , Eq , Copy)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum RoundingError { # [doc = " Error when the TimeDelta exceeds the TimeDelta from or until the Unix epoch."] # [doc = ""] # [doc = " Note: this error is not produced anymore."] DurationExceedsTimestamp , # [doc = " Error when `TimeDelta.num_nanoseconds` exceeds the limit."] # [doc = ""] # [doc = " ``` rust"] # [doc = " # use chrono::{DurationRound, TimeDelta, RoundingError, NaiveDate};"] # [doc = " let dt = NaiveDate::from_ymd_opt(2260, 12, 31)"] # [doc = "     .unwrap()"] # [doc = "     .and_hms_nano_opt(23, 59, 59, 1_75_500_000)"] # [doc = "     .unwrap()"] # [doc = "     .and_utc();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     dt.duration_round(TimeDelta::try_days(300 * 365).unwrap()),"] # [doc = "     Err(RoundingError::DurationExceedsLimit)"] # [doc = " );"] # [doc = " ```"] DurationExceedsLimit , # [doc = " Error when `DateTime.timestamp_nanos` exceeds the limit."] # [doc = ""] # [doc = " ``` rust"] # [doc = " # use chrono::{DurationRound, TimeDelta, RoundingError, TimeZone, Utc};"] # [doc = " let dt = Utc.with_ymd_and_hms(2300, 12, 12, 0, 0, 0).unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     dt.duration_round(TimeDelta::try_days(1).unwrap()),"] # [doc = "     Err(RoundingError::TimestampExceedsLimit)"] # [doc = " );"] # [doc = " ```"] TimestampExceedsLimit , }
};
}
