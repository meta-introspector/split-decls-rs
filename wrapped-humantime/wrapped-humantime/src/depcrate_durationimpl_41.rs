// Generated macro for impl_41 (impl)
macro_rules! Depcrate_durationimpl_41 {
() => {
// Module: crate::duration
// Provides: {"impl_41"}
// Dependencies: {}
impl FromStr for Unit { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "nanos" | "nsec" | "ns" => Ok (Self :: Nanosecond) , "usec" | "us" | "µs" => Ok (Self :: Microsecond) , "millis" | "msec" | "ms" => Ok (Self :: Millisecond) , "seconds" | "second" | "secs" | "sec" | "s" => Ok (Self :: Second) , "minutes" | "minute" | "min" | "mins" | "m" => Ok (Self :: Minute) , "hours" | "hour" | "hr" | "hrs" | "h" => Ok (Self :: Hour) , "days" | "day" | "d" => Ok (Self :: Day) , "weeks" | "week" | "wk" | "wks" | "w" => Ok (Self :: Week) , "months" | "month" | "M" => Ok (Self :: Month) , "years" | "year" | "yr" | "yrs" | "y" => Ok (Self :: Year) , _ => Err (()) , } } }
};
}
