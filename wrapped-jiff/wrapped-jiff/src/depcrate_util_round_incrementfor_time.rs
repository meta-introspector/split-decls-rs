// Generated macro for for_time (function)
macro_rules! Depcrate_util_round_incrementfor_time {
() => {
// Module: crate::util::round::increment
// Provides: {"for_time"}
// Dependencies: {}
# [doc = " Validates the given rounding increment for the given unit."] # [doc = ""] # [doc = " This validation ensures the rounding increment is valid for rounding"] # [doc = " civil times."] pub (crate) fn for_time (unit : Unit , increment : i64 ,) -> Result < t :: NoUnits128 , Error > { static LIMIT : & [Constant] = & [t :: NANOS_PER_MICRO , t :: MICROS_PER_MILLI , t :: MILLIS_PER_SECOND , t :: SECONDS_PER_MINUTE , t :: MINUTES_PER_HOUR , t :: HOURS_PER_CIVIL_DAY ,] ; get_with_limit (unit , increment , "time" , LIMIT) }
};
}
