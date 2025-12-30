// Generated macro for Duration (struct)
macro_rules! Depcrate_duration_durationDuration {
() => {
// Module: crate::duration::duration
// Provides: {"Duration"}
// Dependencies: {}
# [doc = " Represents a duration of time (intuitively, how long something took / will take)."] # [doc = " Can be constructed ergonomically using the [`Default`] trait like so:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use icu_experimental::duration::Duration;"] # [doc = " let d = Duration {"] # [doc = "     years: 1,"] # [doc = "     months: 2,"] # [doc = "     weeks: 3,"] # [doc = "     ..Default::default()"] # [doc = " };"] # [doc = " ```"] # [allow (clippy :: exhaustive_structs)] # [derive (Debug , Default , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct Duration { # [doc = " Whether the duration is positive."] pub sign : DurationSign , # [doc = " The number of years in the duration."] pub years : u64 , # [doc = " The number of months in the duration."] pub months : u64 , # [doc = " The number of weeks in the duration."] pub weeks : u64 , # [doc = " The number of days in the duration."] pub days : u64 , # [doc = " The number of hours in the duration."] pub hours : u64 , # [doc = " The number of minutes in the duration."] pub minutes : u64 , # [doc = " The number of seconds in the duration."] pub seconds : u64 , # [doc = " The number of milliseconds in the duration."] pub milliseconds : u64 , # [doc = " The number of microseconds in the duration."] pub microseconds : u64 , # [doc = " The number of nanoseconds in the duration."] pub nanoseconds : u64 , }
};
}
