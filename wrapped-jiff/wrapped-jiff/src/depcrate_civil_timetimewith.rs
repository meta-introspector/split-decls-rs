// Generated macro for TimeWith (struct)
macro_rules! Depcrate_civil_timeTimeWith {
() => {
// Module: crate::civil::time
// Provides: {"TimeWith"}
// Dependencies: {}
# [doc = " A builder for setting the fields on a [`Time`]."] # [doc = ""] # [doc = " This builder is constructed via [`Time::with`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Unlike [`Date`], a [`Time`] is valid for all possible valid values of its"] # [doc = " fields. That is, there is no way for two valid field values to combine"] # [doc = " into an invalid `Time`. So, for `Time`, this builder does have as much of"] # [doc = " a benefit versus an API design with methods like `Time::with_hour` and"] # [doc = " `Time::with_minute`. Nevertheless, this builder permits settings multiple"] # [doc = " fields at the same time and performing only one validity check. Moreover,"] # [doc = " this provides a consistent API with other date and time types in this"] # [doc = " crate."] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::time;"] # [doc = ""] # [doc = " let t1 = time(0, 0, 24, 0);"] # [doc = " let t2 = t1.with().hour(15).minute(30).millisecond(10).build()?;"] # [doc = " assert_eq!(t2, time(15, 30, 24, 10_000_000));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct TimeWith { original : Time , hour : Option < i8 > , minute : Option < i8 > , second : Option < i8 > , millisecond : Option < i16 > , microsecond : Option < i16 > , nanosecond : Option < i16 > , subsec_nanosecond : Option < i32 > , }
};
}
